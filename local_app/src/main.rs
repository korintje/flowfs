//! A simple example of hooking up stdin/stdout to a WebSocket stream.
//!
//! This example will connect to a server specified in the argument list and
//! then forward all data read on stdin to the server, printing out all data
//! received on stdout.
//!
//! Note that this is not currently optimized for performance, especially around
//! buffer management. Rather it's intended to show an example of working with a
//! client.
//!
//! You can use this example together with the `server` example.

use std::env;

use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::sync::mpsc;
use mac_address::get_mac_address;

mod model;
use model::*;

#[tokio::main]
async fn main() {
    let connect_addr =
        env::args().nth(1).unwrap_or_else(|| panic!("this program requires at least one argument"));

    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {

    /*
    let user_name = "Furoh";
    let create_user_req = CreateUserReq {
        name: user_name.to_string(),
        passhash: "1233456789abc".to_string(),
    };
    let create_usr_req_str = serde_json::to_string(&create_user_req).unwrap();
    let header: u8 = 0;
    let mut u8_vec: Vec<u8> = vec![header];
    u8_vec.extend(create_usr_req_str.as_bytes());
    tx.unbounded_send(u8_vec.into()).unwrap();
    */
    println!("HERE");
    
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