// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::env;
use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use mac_address::get_mac_address;

mod model;
use model::*;

#[tokio::main]
async fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let connect_addr = env::args().nth(1).unwrap_or_else(|| panic!("requires at least one argument"));
    let url = url::Url::parse(&connect_addr).unwrap();

    // 標準入力チャネル
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    // Websocketに接続
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    // websocket streamをreadとwriteに分割
    let (write, read) = ws_stream.split();

    // 標準入力(stdin_rx）からの入力をwriteにforward
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);

    // websocket readから標準出力に書き出し
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    // 非同期の処理をピン留めし、メモリから動的に再配置されないようにします。
    pin_mut!(stdin_to_ws, ws_to_stdout);

    // は、stdin_to_wsとws_to_stdoutの両方の非同期処理が完了するまで待ち、最初に完了した方の処理を続行します。
    future::select(stdin_to_ws, ws_to_stdout).await;

}


async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    
    let stdin = BufReader::new(tokio::io::stdin());

    let mut lines = stdin.lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        let line = line.trim();

        match line {
            "0" => {
                println!("Command 0");
                let device_name = std::env::var("HOSTNAME").unwrap_or("Unknown".to_string());
                let mac_address = get_mac_address().unwrap();
                let _id: [u8; 6] = mac_address.unwrap().bytes();
                let new_device = Device { _id, name: device_name };
                let req_str = serde_json::to_string(&new_device).unwrap();
                tx.unbounded_send(req_str.into()).unwrap();
            }
            "1" => {
                println!("Command 1");
                let user_name = "Mukia Tomei".to_string();
                let passhash = "1234abcd".to_string();
                let _id = mongodb::bson::oid::ObjectId::new();
                let new_user = User { _id, name: user_name, passhash, device_ids: vec![] };
                let req_str = serde_json::to_string(&new_user).unwrap();
                tx.unbounded_send(req_str.into()).unwrap();
            }
            _ => {
                println!("Command n");
            }
        }
    }
    
}