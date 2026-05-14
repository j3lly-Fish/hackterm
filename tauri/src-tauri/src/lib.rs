mod filesystem;
mod network;
mod settings;
mod ssh;
mod sysinfo;
mod terminal;

use notify::RecommendedWatcher;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub struct AppState {
    pub terminals: Mutex<HashMap<u16, terminal::TerminalSession>>,
    pub ssh_sessions: Arc<Mutex<HashMap<String, ssh::SshSession>>>,
    pub sysinfo: Mutex<::sysinfo::System>,
    pub fs_watchers: Mutex<HashMap<String, RecommendedWatcher>>,
}

impl AppState {
    fn new() -> Self {
        use ::sysinfo::System;
        Self {
            terminals: Mutex::new(HashMap::new()),
            ssh_sessions: Arc::new(Mutex::new(HashMap::new())),
            sysinfo: Mutex::new(System::new_all()),
            fs_watchers: Mutex::new(HashMap::new()),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Plugins
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        // Shared state
        .manage(AppState::new())
        // Commands
        .invoke_handler(tauri::generate_handler![
            // Terminal
            terminal::spawn_terminal,
            terminal::resize_terminal,
            terminal::close_terminal,
            terminal::get_terminal_cwd,
            // Filesystem
            filesystem::list_dir,
            filesystem::read_file,
            filesystem::write_file,
            filesystem::path_exists,
            filesystem::watch_dir,
            filesystem::unwatch_dir,
            filesystem::get_block_devices,
            // Sysinfo
            sysinfo::get_cpu_load,
            sysinfo::get_cpu_static,
            sysinfo::get_cpu_temp,
            sysinfo::get_memory,
            sysinfo::get_processes,
            sysinfo::get_network_interfaces,
            sysinfo::get_network_stats,
            sysinfo::get_battery,
            sysinfo::get_system_info,
            sysinfo::get_hardware_info,
            sysinfo::get_network_connections,
            // Network
            network::get_external_ip,
            network::get_geolocation,
            network::ping,
            // SSH
            ssh::spawn_ssh,
            ssh::close_ssh,
            // Settings
            settings::load_settings,
            settings::save_settings,
            settings::load_shortcuts,
            settings::save_shortcuts,
            settings::get_config_dir,
            settings::mirror_assets,
            settings::load_window_state,
            settings::save_window_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running eDEX-UI");
}
