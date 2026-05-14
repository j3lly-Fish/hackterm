use serde::Serialize;
use std::net::IpAddr;
use std::path::Path;

#[derive(Serialize)]
pub struct GeoLocation {
    pub ip: String,
    pub country: String,
    pub country_code: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[tauri::command]
pub async fn get_external_ip() -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get("http://myexternalip.com/raw")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    Ok(resp.trim().to_string())
}

#[tauri::command]
pub async fn get_geolocation(
    app: tauri::AppHandle,
    ip: String,
) -> Result<GeoLocation, String> {
    // Look for GeoLite2-Country.mmdb in app config dir
    let config_dir = tauri::Manager::path(&app)
        .app_config_dir()
        .map_err(|e| e.to_string())?;

    let db_path = config_dir.join("GeoLite2-Country.mmdb");

    if !db_path.exists() {
        // Fall back to City DB
        let city_path = config_dir.join("GeoLite2-City.mmdb");
        if city_path.exists() {
            return lookup_city(&city_path, &ip);
        }
        return Err("No GeoLite2 database found. Place GeoLite2-Country.mmdb or GeoLite2-City.mmdb in the app config directory.".to_string());
    }

    lookup_country(&db_path, &ip)
}

fn lookup_country(db_path: &Path, ip: &str) -> Result<GeoLocation, String> {
    let reader = maxminddb::Reader::open_readfile(db_path).map_err(|e| e.to_string())?;
    let addr: IpAddr = ip.parse().map_err(|e: std::net::AddrParseError| e.to_string())?;
    let country: maxminddb::geoip2::Country = reader.lookup(addr).map_err(|e| e.to_string())?;

    let country_name = country
        .country
        .as_ref()
        .and_then(|c| c.names.as_ref())
        .and_then(|n| n.get("en"))
        .map(|s| s.to_string())
        .unwrap_or_default();

    let country_code = country
        .country
        .as_ref()
        .and_then(|c| c.iso_code)
        .map(|s| s.to_string())
        .unwrap_or_default();

    Ok(GeoLocation {
        ip: ip.to_string(),
        country: country_name,
        country_code,
        city: String::new(),
        latitude: 0.0,
        longitude: 0.0,
    })
}

fn lookup_city(db_path: &Path, ip: &str) -> Result<GeoLocation, String> {
    let reader = maxminddb::Reader::open_readfile(db_path).map_err(|e| e.to_string())?;
    let addr: IpAddr = ip.parse().map_err(|e: std::net::AddrParseError| e.to_string())?;
    let city: maxminddb::geoip2::City = reader.lookup(addr).map_err(|e| e.to_string())?;

    let country_name = city
        .country
        .as_ref()
        .and_then(|c| c.names.as_ref())
        .and_then(|n| n.get("en"))
        .map(|s| s.to_string())
        .unwrap_or_default();

    let country_code = city
        .country
        .as_ref()
        .and_then(|c| c.iso_code)
        .map(|s| s.to_string())
        .unwrap_or_default();

    let city_name = city
        .city
        .as_ref()
        .and_then(|c| c.names.as_ref())
        .and_then(|n| n.get("en"))
        .map(|s| s.to_string())
        .unwrap_or_default();

    let (lat, lon) = city
        .location
        .as_ref()
        .and_then(|l| {
            let lat = l.latitude?;
            let lon = l.longitude?;
            Some((lat, lon))
        })
        .unwrap_or((0.0, 0.0));

    Ok(GeoLocation {
        ip: ip.to_string(),
        country: country_name,
        country_code,
        city: city_name,
        latitude: lat,
        longitude: lon,
    })
}

#[derive(Serialize)]
pub struct PingResult {
    pub addr: String,
    pub latency_ms: Option<u64>,
    pub success: bool,
}

/// TCP echo ping — connects to port 80 and measures RTT.
/// Falls back to port 443 if 80 is filtered.
#[tauri::command]
pub async fn ping(addr: String) -> PingResult {
    use tokio::net::TcpStream;
    use tokio::time::timeout;

    let target = format!("{}:80", addr);
    let start = std::time::Instant::now();

    let result = timeout(
        std::time::Duration::from_secs(3),
        TcpStream::connect(&target),
    )
    .await;

    match result {
        Ok(Ok(_)) => PingResult {
            addr: addr.clone(),
            latency_ms: Some(start.elapsed().as_millis() as u64),
            success: true,
        },
        _ => {
            // Try port 443
            let target443 = format!("{}:443", addr);
            let start2 = std::time::Instant::now();
            let result2 = timeout(
                std::time::Duration::from_secs(3),
                TcpStream::connect(&target443),
            )
            .await;

            match result2 {
                Ok(Ok(_)) => PingResult {
                    addr,
                    latency_ms: Some(start2.elapsed().as_millis() as u64),
                    success: true,
                },
                _ => PingResult {
                    addr,
                    latency_ms: None,
                    success: false,
                },
            }
        }
    }
}
