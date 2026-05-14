use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use russh::client;
use russh_keys::key::PublicKey;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use uuid::Uuid;

pub struct SshSession {
    pub shutdown_tx: Option<oneshot::Sender<()>>,
}

pub type SshSessionMap = HashMap<String, SshSession>;

struct SshClientHandler;

#[async_trait]
impl client::Handler for SshClientHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &PublicKey,
    ) -> Result<bool, Self::Error> {
        // Accept all server keys — add fingerprint verification in a later iteration
        Ok(true)
    }
}

#[derive(Serialize)]
pub struct SshSpawnResult {
    pub session_id: String,
}

#[tauri::command]
pub async fn spawn_ssh(
    state: tauri::State<'_, crate::AppState>,
    host: String,
    port: u16,
    username: String,
    password: String,
    ws_port: u16,
    cols: u32,
    rows: u32,
) -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let sid = session_id.clone();
    let state_inner = state.ssh_sessions.clone();

    tokio::spawn(async move {
        if let Err(e) = run_ssh_session(
            host,
            port,
            username,
            password,
            ws_port,
            cols,
            rows,
            shutdown_rx,
        )
        .await
        {
            eprintln!("SSH session {} error: {}", sid, e);
        }

        // Clean up from state when session ends
        let mut sessions = state_inner.lock().unwrap();
        sessions.remove(&sid);
    });

    let mut sessions = state.ssh_sessions.lock().unwrap();
    sessions.insert(
        session_id.clone(),
        SshSession {
            shutdown_tx: Some(shutdown_tx),
        },
    );

    Ok(session_id)
}

async fn run_ssh_session(
    host: String,
    port: u16,
    username: String,
    password: String,
    ws_port: u16,
    cols: u32,
    rows: u32,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Connect to SSH server
    let config = Arc::new(client::Config::default());
    let mut session = client::connect(config, (host.as_str(), port), SshClientHandler).await?;

    let authenticated = session
        .authenticate_password(&username, &password)
        .await?;

    if !authenticated {
        return Err("SSH authentication failed".into());
    }

    let channel = session.channel_open_session().await?;
    channel
        .request_pty(
            false,
            "xterm-256color",
            cols,
            rows,
            0,
            0,
            &[],
        )
        .await?;
    channel.request_shell(false).await?;

    // Channel I/O
    let (ssh_tx, mut ssh_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    let (ws_in_tx, mut ws_in_rx) = mpsc::unbounded_channel::<Vec<u8>>();

    // WebSocket server for xterm.js
    let addr = format!("127.0.0.1:{}", ws_port);
    let listener = TcpListener::bind(&addr).await?;
    let (stream, _) = listener.accept().await?;
    let ws = accept_async(stream).await?;
    let (mut ws_write, mut ws_read) = ws.split();

    // Forward SSH → WebSocket
    let ssh_tx_clone = ssh_tx.clone();
    tokio::spawn(async move {
        use russh::ChannelMsg;
        loop {
            // channel.wait() is not available; instead read from channel stream
            // This will be implemented differently based on russh API
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    });

    loop {
        tokio::select! {
            // SSH output → WebSocket
            Some(bytes) = ssh_rx.recv() => {
                let _ = ws_write.send(Message::Binary(bytes)).await;
            }
            // WebSocket input → SSH
            msg = ws_read.next() => {
                match msg {
                    Some(Ok(Message::Binary(data))) => {
                        channel.data(data.as_ref()).await?;
                    }
                    Some(Ok(Message::Text(text))) => {
                        channel.data(text.as_bytes()).await?;
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
            _ = &mut shutdown_rx => break,
        }
    }

    channel.close().await?;
    session.disconnect(russh::Disconnect::ByApplication, "", "en").await?;
    Ok(())
}

#[tauri::command]
pub fn close_ssh(
    state: tauri::State<'_, crate::AppState>,
    session_id: String,
) -> Result<(), String> {
    let mut sessions = state.ssh_sessions.lock().unwrap();
    if let Some(mut session) = sessions.remove(&session_id) {
        if let Some(tx) = session.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
    Ok(())
}
