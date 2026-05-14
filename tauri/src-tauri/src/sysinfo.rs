use serde::Serialize;
use sysinfo::{Components, Disks, Networks, ProcessRefreshKind, ProcessesToUpdate, System};

// ──────────────────────── CPU ────────────────────────

#[derive(Serialize)]
pub struct CpuLoad {
    pub global: f32,
    pub per_core: Vec<f32>,
}

#[tauri::command]
pub fn get_cpu_load(state: tauri::State<'_, crate::AppState>) -> CpuLoad {
    let mut sys = state.sysinfo.lock().unwrap();
    sys.refresh_cpu_all();
    CpuLoad {
        global: sys.global_cpu_usage(),
        per_core: sys.cpus().iter().map(|c| c.cpu_usage()).collect(),
    }
}

#[derive(Serialize)]
pub struct CpuStatic {
    pub brand: String,
    pub cores: usize,
    pub logical: usize,
    pub frequency: u64,
}

#[tauri::command]
pub fn get_cpu_static(state: tauri::State<'_, crate::AppState>) -> CpuStatic {
    let sys = state.sysinfo.lock().unwrap();
    let cpus = sys.cpus();
    let brand = cpus.first().map(|c| c.brand().to_string()).unwrap_or_default();
    let freq = cpus.first().map(|c| c.frequency()).unwrap_or(0);
    CpuStatic {
        brand,
        cores: sys.physical_core_count().unwrap_or(0),
        logical: cpus.len(),
        frequency: freq,
    }
}

#[derive(Serialize)]
pub struct CpuTemp {
    pub label: String,
    pub temperature: f32,
}

#[tauri::command]
pub fn get_cpu_temp() -> Vec<CpuTemp> {
    let components = Components::new_with_refreshed_list();
    components
        .iter()
        .map(|c| CpuTemp {
            label: c.label().to_string(),
            temperature: c.temperature(),
        })
        .collect()
}

// ──────────────────────── Memory ────────────────────────

#[derive(Serialize)]
pub struct Memory {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

#[tauri::command]
pub fn get_memory(state: tauri::State<'_, crate::AppState>) -> Memory {
    let mut sys = state.sysinfo.lock().unwrap();
    sys.refresh_memory();
    Memory {
        total: sys.total_memory(),
        used: sys.used_memory(),
        free: sys.free_memory(),
        swap_total: sys.total_swap(),
        swap_used: sys.used_swap(),
    }
}

// ──────────────────────── Processes ────────────────────────

#[derive(Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
    pub status: String,
}

#[tauri::command]
pub fn get_processes(state: tauri::State<'_, crate::AppState>) -> Vec<ProcessInfo> {
    let mut sys = state.sysinfo.lock().unwrap();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    let mut procs: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, p)| ProcessInfo {
            pid: pid.as_u32(),
            name: p.name().to_string_lossy().to_string(),
            cpu: p.cpu_usage(),
            memory: p.memory(),
            status: format!("{:?}", p.status()),
        })
        .collect();
    procs.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));
    procs
}

// ──────────────────────── Network ────────────────────────

#[derive(Serialize, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub ip4: Vec<String>,
    pub ip6: Vec<String>,
    pub mac: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_speed: u64,
    pub tx_speed: u64,
}

#[tauri::command]
pub fn get_network_interfaces() -> Vec<NetworkInterface> {
    use sysinfo::Networks;
    let networks = Networks::new_with_refreshed_list();
    networks
        .iter()
        .map(|(name, net)| NetworkInterface {
            name: name.clone(),
            ip4: net
                .ip_networks()
                .iter()
                .filter(|a| a.addr.is_ipv4())
                .map(|a| a.addr.to_string())
                .collect(),
            ip6: net
                .ip_networks()
                .iter()
                .filter(|a| a.addr.is_ipv6())
                .map(|a| a.addr.to_string())
                .collect(),
            mac: net.mac_address().to_string(),
            rx_bytes: net.total_received(),
            tx_bytes: net.total_transmitted(),
            rx_speed: net.received(),
            tx_speed: net.transmitted(),
        })
        .collect()
}

#[derive(Serialize)]
pub struct NetworkStats {
    pub name: String,
    pub rx: u64,
    pub tx: u64,
    pub rx_total: u64,
    pub tx_total: u64,
}

#[tauri::command]
pub fn get_network_stats(iface: String) -> Option<NetworkStats> {
    let mut networks = Networks::new_with_refreshed_list();
    networks.refresh();
    networks.get(&iface).map(|net| NetworkStats {
        name: iface.clone(),
        rx: net.received(),
        tx: net.transmitted(),
        rx_total: net.total_received(),
        tx_total: net.total_transmitted(),
    })
}

// ──────────────────────── Battery ────────────────────────

#[derive(Serialize)]
pub struct BatteryInfo {
    pub charge: f32,
    pub state: String,
    pub time_to_full: Option<f32>,
    pub time_to_empty: Option<f32>,
}

#[tauri::command]
pub fn get_battery() -> Option<BatteryInfo> {
    let manager = battery::Manager::new().ok()?;
    let mut batteries = manager.batteries().ok()?;
    let bat = batteries.next()?.ok()?;
    Some(BatteryInfo {
        charge: bat.state_of_charge().value * 100.0,
        state: format!("{:?}", bat.state()),
        time_to_full: bat.time_to_full().map(|t| t.value),
        time_to_empty: bat.time_to_empty().map(|t| t.value),
    })
}

// ──────────────────────── System info ────────────────────────

#[derive(Serialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub uptime: u64,
    pub boot_time: u64,
}

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        hostname: System::host_name().unwrap_or_default(),
        os_name: System::name().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
        kernel_version: System::kernel_version().unwrap_or_default(),
        uptime: System::uptime(),
        boot_time: System::boot_time(),
    }
}

// ──────────────────────── Hardware ────────────────────────

#[derive(Serialize)]
pub struct HardwareInfo {
    pub cpu_brand: String,
    pub cpu_cores: usize,
    pub total_memory: u64,
    pub disks: Vec<DiskInfo>,
}

#[derive(Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount: String,
    pub total: u64,
    pub available: u64,
}

#[tauri::command]
pub fn get_hardware_info(state: tauri::State<'_, crate::AppState>) -> HardwareInfo {
    let sys = state.sysinfo.lock().unwrap();
    let cpu_brand = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_default();
    let disks = Disks::new_with_refreshed_list();
    HardwareInfo {
        cpu_brand,
        cpu_cores: sys.physical_core_count().unwrap_or(0),
        total_memory: sys.total_memory(),
        disks: disks
            .iter()
            .map(|d| DiskInfo {
                name: d.name().to_string_lossy().to_string(),
                mount: d.mount_point().to_string_lossy().to_string(),
                total: d.total_space(),
                available: d.available_space(),
            })
            .collect(),
    }
}

// ──────────────────────── Network connections ────────────────────────

#[derive(Serialize)]
pub struct NetworkConnection {
    pub protocol: String,
    pub local_addr: String,
    pub local_port: u16,
    pub remote_addr: String,
    pub remote_port: u16,
    pub state: String,
    pub pid: Option<u32>,
}

#[tauri::command]
pub async fn get_network_connections() -> Vec<NetworkConnection> {
    // sysinfo doesn't expose socket-level connections; use ss/netstat via command
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let output = tokio::process::Command::new("ss")
            .args(["-tunp"])
            .output()
            .await;

        if let Ok(out) = output {
            let raw = String::from_utf8_lossy(&out.stdout);
            return parse_ss_output(&raw);
        }
    }
    vec![]
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn parse_ss_output(raw: &str) -> Vec<NetworkConnection> {
    let mut result = Vec::new();
    for line in raw.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            continue;
        }
        let proto = parts[0].to_string();
        let state = parts[1].to_string();
        let local = parts[4];
        let remote = parts.get(5).unwrap_or(&"").to_string();

        let (local_addr, local_port) = split_addr_port(local);
        let (remote_addr, remote_port) = split_addr_port(&remote);

        result.push(NetworkConnection {
            protocol: proto,
            local_addr,
            local_port,
            remote_addr,
            remote_port,
            state,
            pid: None,
        });
    }
    result
}

fn split_addr_port(s: &str) -> (String, u16) {
    if let Some(pos) = s.rfind(':') {
        let addr = s[..pos].trim_matches(|c| c == '[' || c == ']').to_string();
        let port = s[pos + 1..].parse().unwrap_or(0);
        (addr, port)
    } else {
        (s.to_string(), 0)
    }
}
