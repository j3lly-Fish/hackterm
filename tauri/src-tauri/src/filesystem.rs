use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::path::Path;
use std::collections::HashMap;
use tauri::Emitter;

#[derive(Serialize, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified: u64,
    pub kind: String, // "file" | "dir" | "symlink"
    pub is_symlink: bool,
}

#[derive(Serialize, Clone)]
pub struct FsChangeEvent {
    pub path: String,
    pub kind: String,
}

pub type WatcherMap = HashMap<String, RecommendedWatcher>;

#[tauri::command]
pub async fn list_dir(path: String) -> Result<Vec<FileEntry>, String> {
    let entries = std::fs::read_dir(&path).map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    for entry in entries.flatten() {
        let metadata = entry.metadata().map_err(|e| e.to_string())?;
        let file_path = entry.path().to_string_lossy().to_string();
        let name = entry.file_name().to_string_lossy().to_string();
        let size = metadata.len();
        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let kind = if metadata.is_symlink() {
            "symlink"
        } else if metadata.is_dir() {
            "dir"
        } else {
            "file"
        }
        .to_string();

        result.push(FileEntry {
            name,
            path: file_path,
            size,
            modified,
            kind,
            is_symlink: metadata.is_symlink(),
        });
    }

    Ok(result)
}

#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn path_exists(path: String) -> bool {
    Path::new(&path).exists()
}

#[tauri::command]
pub fn watch_dir(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::AppState>,
    path: String,
) -> Result<(), String> {
    let path_clone = path.clone();
    let app_clone = app.clone();

    let watcher = RecommendedWatcher::new(
        move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                let kind = format!("{:?}", event.kind);
                let _ = app_clone.emit(
                    "fs-change",
                    FsChangeEvent {
                        path: path_clone.clone(),
                        kind,
                    },
                );
            }
        },
        Config::default(),
    )
    .map_err(|e| e.to_string())?;

    // The watcher is stored so it keeps watching until explicitly removed
    let mut watchers = state.fs_watchers.lock().unwrap();

    // If we're already watching this path, replace the watcher
    watchers.remove(&path);

    let mut w = watcher;
    w.watch(Path::new(&path), RecursiveMode::NonRecursive)
        .map_err(|e| e.to_string())?;

    watchers.insert(path, w);
    Ok(())
}

#[tauri::command]
pub fn unwatch_dir(
    state: tauri::State<'_, crate::AppState>,
    path: String,
) -> Result<(), String> {
    let mut watchers = state.fs_watchers.lock().unwrap();
    watchers.remove(&path);
    Ok(())
}

#[derive(Serialize)]
pub struct BlockDevice {
    pub name: String,
    pub mount: String,
    pub total: u64,
    pub available: u64,
    pub kind: String,
}

#[tauri::command]
pub fn get_block_devices(
    state: tauri::State<'_, crate::AppState>,
) -> Vec<BlockDevice> {
    use sysinfo::Disks;
    let disks = Disks::new_with_refreshed_list();
    disks
        .iter()
        .map(|d| BlockDevice {
            name: d.name().to_string_lossy().to_string(),
            mount: d.mount_point().to_string_lossy().to_string(),
            total: d.total_space(),
            available: d.available_space(),
            kind: format!("{:?}", d.kind()),
        })
        .collect()
}
