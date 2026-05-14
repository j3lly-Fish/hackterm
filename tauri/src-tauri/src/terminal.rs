use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use serde::{Deserialize, Serialize};

pub struct TerminalSession {
    pub child_pid: u32,
    pub master: Arc<Mutex<Box<dyn portable_pty::MasterPty + Send>>>,
    pub write_tx: mpsc::UnboundedSender<Vec<u8>>,
    pub shutdown_tx: Option<oneshot::Sender<()>>,
}

pub type TerminalMap = HashMap<u16, TerminalSession>;

#[derive(Serialize)]
pub struct SpawnResult {
    pub pid: u32,
}

#[derive(Deserialize)]
pub struct SpawnArgs {
    pub shell: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub port: u16,
    pub cols: u16,
    pub rows: u16,
}

#[tauri::command]
pub async fn spawn_terminal(
    state: tauri::State<'_, crate::AppState>,
    shell: String,
    args: Vec<String>,
    cwd: String,
    port: u16,
    cols: u16,
    rows: u16,
) -> Result<u32, String> {
    let pty_system = native_pty_system();

    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| e.to_string())?;

    let mut cmd = CommandBuilder::new(&shell);
    for arg in &args {
        cmd.arg(arg);
    }
    if !cwd.is_empty() {
        cmd.cwd(&cwd);
    }
    // Inherit environment so the shell starts with the user's env
    cmd.env("TERM", "xterm-256color");

    let child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;
    let child_pid = child.process_id().unwrap_or(0);

    // Must drop slave after spawning to avoid holding open the other end
    drop(pair.slave);

    // Take reader/writer from master before wrapping in Arc<Mutex>
    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| e.to_string())?;
    let mut writer = pair
        .master
        .take_writer()
        .map_err(|e| e.to_string())?;

    let master = Arc::new(Mutex::new(pair.master));

    // PTY read → WebSocket write
    let (pty_tx, mut pty_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    // WebSocket read → PTY write
    let (ws_tx, mut ws_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    // Shutdown signal
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // Thread: read from PTY, forward to async channel
    let pty_tx_clone = pty_tx.clone();
    std::thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if pty_tx_clone.send(buf[..n].to_vec()).is_err() {
                        break;
                    }
                }
            }
        }
    });

    // Thread: receive from async channel, write to PTY
    std::thread::spawn(move || {
        // Drive ws_rx from a new single-threaded tokio runtime on this thread
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async move {
            while let Some(data) = ws_rx.recv().await {
                if writer.write_all(&data).is_err() {
                    break;
                }
            }
        });
    });

    // If a session already exists on this port (e.g. hot-reload), shut it down first
    // and wait for the OS to release the port before re-binding.
    {
        let mut terminals = state.terminals.lock().unwrap();
        if let Some(mut old) = terminals.remove(&port) {
            if let Some(tx) = old.shutdown_tx.take() {
                let _ = tx.send(());
            }
        }
    }
    // Give the old task time to exit and release the port (shutdown_rx cancels accept())
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Bind the TCP listener *before* returning so the frontend can connect immediately
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await.map_err(|e| e.to_string())?;

    // Async task: WebSocket server
    let ws_tx_clone = ws_tx.clone();
    tokio::spawn(async move {
        let mut shutdown_rx = shutdown_rx;

        // Accept connection — cancellable so the port is released immediately on shutdown/reload
        let (stream, _) = tokio::select! {
            result = listener.accept() => match result {
                Ok(s) => s,
                Err(e) => { eprintln!("WS accept failed: {}", e); return; }
            },
            _ = &mut shutdown_rx => return,
        };

        let ws = match accept_async(stream).await {
            Ok(ws) => ws,
            Err(e) => {
                eprintln!("WS upgrade failed: {}", e);
                return;
            }
        };

        let (mut ws_write, mut ws_read) = ws.split();

        loop {
            tokio::select! {
                biased;
                // PTY output → WebSocket
                data = pty_rx.recv() => {
                    match data {
                        Some(bytes) => {
                            let _ = ws_write.send(Message::Binary(bytes)).await;
                        }
                        None => break, // PTY closed
                    }
                }
                // WebSocket input → PTY
                msg = ws_read.next() => {
                    match msg {
                        Some(Ok(Message::Binary(data))) => {
                            let _ = ws_tx_clone.send(data);
                        }
                        Some(Ok(Message::Text(text))) => {
                            let _ = ws_tx_clone.send(text.into_bytes());
                        }
                        Some(Ok(Message::Close(_))) | None => break,
                        _ => {}
                    }
                }
                _ = &mut shutdown_rx => break,
            }
        }
    });

    let mut terminals = state.terminals.lock().unwrap();
    terminals.insert(
        port,
        TerminalSession {
            child_pid,
            master,
            write_tx: ws_tx,
            shutdown_tx: Some(shutdown_tx),
        },
    );

    Ok(child_pid)
}

#[tauri::command]
pub fn resize_terminal(
    state: tauri::State<'_, crate::AppState>,
    port: u16,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let terminals = state.terminals.lock().unwrap();
    if let Some(session) = terminals.get(&port) {
        let master = session.master.lock().unwrap();
        master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn close_terminal(
    state: tauri::State<'_, crate::AppState>,
    port: u16,
) -> Result<(), String> {
    let mut terminals = state.terminals.lock().unwrap();
    if let Some(mut session) = terminals.remove(&port) {
        if let Some(tx) = session.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
    Ok(())
}

/// Returns the current working directory of the foreground process in the PTY.
/// Uses platform-specific methods.
#[tauri::command]
pub async fn get_terminal_cwd(pid: u32) -> Result<String, String> {
    #[cfg(target_os = "linux")]
    {
        let link = format!("/proc/{}/cwd", pid);
        std::fs::read_link(&link)
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| e.to_string())
    }
    #[cfg(target_os = "macos")]
    {
        let out = tokio::process::Command::new("lsof")
            .args(["-a", "-d", "cwd", "-p", &pid.to_string(), "-F", "n"])
            .output()
            .await
            .map_err(|e| e.to_string())?;
        let raw = String::from_utf8_lossy(&out.stdout);
        // lsof -F n output: lines starting with 'n' contain the path
        for line in raw.lines() {
            if line.starts_with('n') && line.len() > 1 {
                return Ok(line[1..].to_string());
            }
        }
        Err("cwd not found".to_string())
    }
    #[cfg(target_os = "windows")]
    {
        Err("cwd tracking not yet implemented on Windows".to_string())
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err("unsupported platform".to_string())
    }
}
