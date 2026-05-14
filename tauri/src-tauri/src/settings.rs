use std::path::PathBuf;
use tauri::Manager;

fn config_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_config_dir()
        .map_err(|e| e.to_string())
}

// ──────────────────────── Default settings ────────────────────────

fn default_shell() -> String {
    #[cfg(target_os = "windows")]
    return "powershell.exe".to_string();
    #[cfg(not(target_os = "windows"))]
    {
        // Try to use the user's login shell
        std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string())
    }
}

fn default_cwd() -> String {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| "/".to_string())
}

fn default_settings(_config_dir: &PathBuf) -> serde_json::Value {
    serde_json::json!({
        "shell": default_shell(),
        "shellArgs": "",
        "cwd": default_cwd(),
        "keyboard": "en-US",
        "theme": "tron",
        "termFontSize": 15,
        "audio": true,
        "audioVolume": 1.0,
        "disableFeedbackAudio": false,
        "clockHours": 24,
        "pingAddr": "1.1.1.1",
        "port": 3000,
        "nointro": false,
        "nocursor": false,
        "forceFullscreen": true,
        "allowWindowed": false,
        "excludeThreadsFromToplist": true,
        "hideDotfiles": false,
        "fsListView": false,
        "experimentalGlobeFeatures": false,
        "experimentalFeatures": false,
        "splitPanes": false,
        "splitRatio": 0.5,
        "sshProfiles": [],
        "alerts": {
            "enabled": true,
            "cpuThreshold": 90,
            "ramThreshold": 85,
            "tempThreshold": 80,
            "cooldownSeconds": 30
        }
    })
}

fn default_shortcuts() -> serde_json::Value {
    serde_json::json!([
        { "name": "COPY",           "key": "C",           "ctrlKey": true, "shiftKey": true },
        { "name": "PASTE",          "key": "V",           "ctrlKey": true, "shiftKey": true },
        { "name": "SETTINGS",       "key": "F12",         "ctrlKey": false, "shiftKey": false },
        { "name": "NEXT_TAB",       "key": "ArrowRight",  "ctrlKey": true, "shiftKey": true },
        { "name": "PREVIOUS_TAB",   "key": "ArrowLeft",   "ctrlKey": true, "shiftKey": true },
        { "name": "KILL_TAB",       "key": "K",           "ctrlKey": true, "shiftKey": true },
        { "name": "SSH_CONNECT",    "key": "S",           "ctrlKey": true, "shiftKey": true },
        { "name": "CLOSE",          "key": "Q",           "ctrlKey": true, "shiftKey": true }
    ])
}

// ──────────────────────── Settings commands ────────────────────────

#[tauri::command]
pub fn load_settings(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let dir = config_dir(&app)?;
    let path = dir.join("settings.json");

    if path.exists() {
        let raw = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let mut loaded: serde_json::Value =
            serde_json::from_str(&raw).map_err(|e| e.to_string())?;

        // Merge in defaults for any missing keys
        let defaults = default_settings(&dir);
        if let (Some(obj), Some(def_obj)) = (loaded.as_object_mut(), defaults.as_object()) {
            for (k, v) in def_obj {
                obj.entry(k).or_insert(v.clone());
            }
        }
        Ok(loaded)
    } else {
        // First run — create config dir and write defaults
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let defaults = default_settings(&dir);
        let json = serde_json::to_string_pretty(&defaults).map_err(|e| e.to_string())?;
        std::fs::write(&path, &json).map_err(|e| e.to_string())?;
        Ok(defaults)
    }
}

#[tauri::command]
pub fn save_settings(app: tauri::AppHandle, settings: serde_json::Value) -> Result<(), String> {
    let dir = config_dir(&app)?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("settings.json");
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_shortcuts(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let dir = config_dir(&app)?;
    let path = dir.join("shortcuts.json");

    if path.exists() {
        let raw = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&raw).map_err(|e| e.to_string())
    } else {
        let defaults = default_shortcuts();
        let json = serde_json::to_string_pretty(&defaults).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        std::fs::write(&path, &json).map_err(|e| e.to_string())?;
        Ok(defaults)
    }
}

#[tauri::command]
pub fn save_shortcuts(app: tauri::AppHandle, shortcuts: serde_json::Value) -> Result<(), String> {
    let dir = config_dir(&app)?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("shortcuts.json");
    let json = serde_json::to_string_pretty(&shortcuts).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_config_dir(app: tauri::AppHandle) -> Result<String, String> {
    config_dir(&app).map(|p| p.to_string_lossy().to_string())
}

/// Copy bundled assets (themes, kb_layouts, fonts) into the app config dir on launch.
/// Files that already exist are NOT overwritten (preserves user customisation).
#[tauri::command]
pub fn mirror_assets(app: tauri::AppHandle) -> Result<(), String> {
    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| e.to_string())?
        .join("assets");

    let config_dir = config_dir(&app)?;

    for sub in &["themes", "kb_layouts", "fonts"] {
        let src = resource_dir.join(sub);
        let dst = config_dir.join(sub);
        if src.exists() {
            copy_dir_no_overwrite(&src, &dst).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

fn copy_dir_no_overwrite(
    src: &PathBuf,
    dst: &PathBuf,
) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)?.flatten() {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if !dst_path.exists() {
            if src_path.is_dir() {
                copy_dir_no_overwrite(&src_path, &dst_path)?;
            } else {
                std::fs::copy(&src_path, &dst_path)?;
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub fn load_window_state(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let dir = config_dir(&app)?;
    let path = dir.join("lastWindowState.json");

    if path.exists() {
        let raw = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&raw).map_err(|e| e.to_string())
    } else {
        Ok(serde_json::json!({ "fullscreen": true }))
    }
}

#[tauri::command]
pub fn save_window_state(
    app: tauri::AppHandle,
    state: serde_json::Value,
) -> Result<(), String> {
    let dir = config_dir(&app)?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("lastWindowState.json");
    let json = serde_json::to_string_pretty(&state).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())
}
