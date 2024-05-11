// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::WindowBuilder;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::connect_async;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot;
use std::net::SocketAddr;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // UIスレッドと通信するチャンネルを作成します
    let (tx_ui, mut rx_ui) = mpsc::channel::<String>(32);

    // WebSocketスレッドと通信するチャンネルを作成します
    let (tx_ws, mut rx_ws) = mpsc::channel::<String>(32);

    // Tauriアプリケーションを開始します
    let app = tauri::Builder::default()
        .invoke_handler(move |arg| {
            let tx_ui = tx_ui.clone();
            tauri::async_runtime::spawn(async move {
                if let Ok(arg) = arg {
                    if arg == "GREET" {
                        // "GREET"メッセージが送信された場合、WebSocketスレッドに"hello"メッセージを送信します
                        tx_ws.send("hello".to_string()).await.unwrap();
                    }
                }
            });
            Ok(())
        })
        .build(tauri::generate_context!());

    // WebSocketスレッドを起動します
    tokio::spawn(async move {
        let server_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

        loop {
            // WebSocketサーバーに接続します
            let (ws_stream, _) = match connect_async(format!("ws://{}/websocket", server_addr)).await {
                Ok((ws_stream, _)) => ws_stream,
                Err(e) => {
                    println!("Failed to connect to server: {}", e);
                    // 接続に失敗した場合、1秒待って再接続を試みます
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            };

            // WebSocketサーバーからのメッセージを処理します
            let (write, read) = ws_stream.split();
            let (tx, mut rx) = mpsc::channel::<Result<WsMessage, tokio_tungstenite::tungstenite::Error>>(32);

            // WebSocketサーバーからのメッセージをUIスレッドに送信します
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if let Ok(msg) = msg {
                        let msg = match msg {
                            WsMessage::Text(text) => text,
                            WsMessage::Binary(bin) => String::from_utf8(bin).unwrap(),
                            _ => continue,
                        };
                        tx_ui.send(msg).await.unwrap();
                    }
                }
            });

            // WebSocketサーバーにメッセージを送信するためのハンドラを作成します
            let (tx_ws_clone, rx_ws_clone) = mpsc::channel::<String>(32);
            tokio::spawn(async move {
                while let Some(msg) = rx_ws_clone.recv().await {
                    if let Err(e) = write.send(WsMessage::Text(msg)).await {
                        println!("Failed to send message: {}", e);
                    }
                }
            });

            // WebSocketサーバーからのメッセージを受信します
            tokio::spawn(async move {
                let mut read = read;
                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(msg) => {
                            tx.send(Ok(msg)).await.unwrap();
                        }
                        Err(e) => {
                            println!("Failed to receive message: {}", e);
                            // 接続が切断された場合、再接続を試みます
                            break;
                        }
                    }
                }
            });

            // UIスレッドからのメッセージを処理します
            while let Some(msg) = rx_ws.recv().await {
                // WebSocketスレッドにメッセージを送信します
                tx_ws_clone.send(msg).await.unwrap();
            }
        }
    });

    // UIを作成します
    app.run(|app_handle| {
        let window = WindowBuilder::new().build(app_handle).unwrap();
        let tx_ui = tx_ui.clone();
        window.listen("update_message", move |_msg| {
            tx_ui.clone().try_send("UPDATED".to_string()).unwrap();
        });

        Ok(())
    });

    Ok(())
}
