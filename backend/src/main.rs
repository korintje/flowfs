use std::{
    collections::HashMap, env, io::{Error as IoError, Read}, net::SocketAddr, sync::{Arc, Mutex}
};

use mongodb::{
    bson::{doc, oid::ObjectId, Document}, options::{FindOneOptions, AggregateOptions}, Client, Collection, Database,
};

mod utils;
mod model;
use model::*;
// mod db_handler;

#[derive(serde::Deserialize, Debug)]
struct LoadProjReq {
    proj_id: u64,
}

#[derive(serde::Deserialize, Debug)]
struct FetchProjReq {}

#[derive(serde::Deserialize, Debug)]
struct LoadCmd {}

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;



async fn handle_connection(peer_map: PeerMap, db: Arc<Database>, raw_stream: TcpStream, addr: SocketAddr) {

    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    // let db2 = Arc::new(db);

    let broadcast_incoming = incoming.for_each(|msg_result| async {

        let msg = msg_result.unwrap();
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
      
        // メッセージを解析し、ファイルとコメントに分割する
        let data = msg.into_data();
        let (cmd_binary, body) = &data.split_at(1);
        let cmd = cmd_binary[0] as u64;
        match cmd {
            0 =>  {
                let req_json: CreateLumpReq = serde_json::from_slice(body).unwrap();
                db.collection("lumps").insert_one(req_json, None).await.unwrap();
            }
            1 => {
                let req: GetLumpPropReq = serde_json::from_slice(body).unwrap();
                let lumps: Collection<Lump> = db.collection("lumps");

                let pipeline: Vec<bson::Document> = vec![
                    doc! {
                        "$graphLookup": {
                            "from": "collection", // 結合対象のコレクション名
                            "startWith": "$parent_id", // 親子関係を示すフィールド
                            "connectFromField": "parent_id", // 親コレクションのフィールド
                            "connectToField": "_id", // 子コレクションのフィールド
                            "as": "children" // 結果を格納するフィールド名
                        }
                    },
                    doc! {
                        "$graphLookup": {
                            "from": "users",
                            "startWith": "$user_id",
                            "connectFromField": "user_id",
                            "connectToField": "_id",
                            "as": "user"
                        }
                    }
                ];

                // Aggregation Pipelineを実行
                let options = AggregateOptions::builder().allow_disk_use(true).build();
                let cursor = lumps.aggregate(pipeline, options).await.unwrap();


                // let options = FindOneOptions::default();
                /*
                let cursor = lumps.find_one(doc! {"_id": req.lump_id}, None).await;
                let Some(lump) = cursor.unwrap() else { return };
                
                let users: Collection<User> = db.collection("users");
                let u_cursor = users.find_one(doc! {"_id": lump.user_id}, None).await;
                let Some(user) = u_cursor.unwrap() else { return };
                let user_name = user.name;
                
                let devices: Collection<Device> = db.collection("devices");
                let dev_cursor = devices.find_one(doc! {"_id": lump.device_id}, None).await;
                let Some(device) = dev_cursor.unwrap() else { return };
                let device_name = device.name;

                let ancestor_ids = lump.ancestor_ids;
                
                let dirs: Collection<Directory> = db.collection("directories");
                for id in lump.dir_ids.iter() {
                    let dir_cursor = dirs.find_one(doc! {"_id": id}, None).await;
                    let Some(dir) = dir_cursor.unwrap() else { return };
                    let dir_name = dir.name;
                }

                let files: Collection<File> = db.collection("files");
                for id in lump.file_ids.iter() {
                    let fil_cursor = files.find_one(doc! {"_id": id}, None).await;
                    let Some(file) = fil_cursor.unwrap() else { return };
                    let file_name = file.name; 
                }
                */

                

            }
            _ => {}
        }
        let (file, comment): (Vec<u8>, String) = serde_json::from_slice(body).expect("Error parsing JSON");

        // We want to broadcast the message to everyone except ourselves.
        let peers = peer_map.lock().unwrap();
        let broadcast_recipients =
            peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

        for recp in broadcast_recipients {
            // recp.unbounded_send(msg.clone()).unwrap();
            recp.unbounded_send("Accepted!".into()).unwrap();
        }

        // future::ok(())
        // Ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {

    // Set web URL
    let listen_url = utils::get_url();
    let addr = env::args().nth(1).unwrap_or_else(|| listen_url);

    // MongoDBクライアントのセットアップ
    // Ref: https://www.mongodb.com/docs/drivers/rust/current/quick-start/connect-to-mongodb/
    let db_url = utils::get_db_path();
    let db_client = mongodb::Client::with_uri_str(db_url).await.unwrap();
    let db = db_client.database("flowfs");
    let coll_user = db.collection::<model::User>("users");
    let coll_devices = db.collection::<model::Device>("devices");
    let coll_lumps = db.collection::<model::Lump>("lumps");
    let coll_dirs = db.collection::<model::Directory>("directories");
    let coll_files = db.collection::<model::File>("files");

    let db_arc = Arc::new(db);

    // Create PeerMap state
    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        let db_clone = Arc::clone(&db_arc);
        tokio::spawn(handle_connection(state.clone(), db_clone, stream, addr));
    }

    Ok(())
}