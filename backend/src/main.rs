use std::{collections::HashMap, env, io, net::SocketAddr, sync::{Arc, Mutex}};
use mongodb::{bson::doc, options::{UpdateOptions, AggregateOptions}, Collection, Database};
use mongodb::bson::oid::ObjectId;
// use futures_channel::mpsc::{unbounded, UnboundedSender};
// use futures_util::{future, pin_mut, StreamExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
// use tokio_tungstenite::tungstenite::protocol::Message;

mod utils;
mod model;
use model::*;

// type Tx = UnboundedSender<Message>;
// type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;


use axum::{
    routing::{get, post, put},
    Router,
    extract::Path,
    response::Json,
};

#[tokio::main]
async fn main() {

    // Set web URL
    let listen_url = utils::get_url();
    let addr = env::args().nth(1).unwrap_or_else(|| listen_url);

    // MongoDBクライアントのセットアップ
    // Ref: https://www.mongodb.com/docs/drivers/rust/current/quick-start/connect-to-mongodb/
    let db_url = utils::get_db_path();
    let db_client = mongodb::Client::with_uri_str(db_url).await.unwrap();
    let db = db_client.database("flowfs");
    let db_arc = Arc::new(db);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(show_user).put(update_user))
        .route("/cells/", get(list_cells).post(create_cell))
        .route("/cells/:id", get(show_cell).put(update_cell));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}


async fn list_users() {}
async fn create_user() {}
async fn show_user(Path(id): Path<ObjectId>) -> Json<ShowUserRes> {
    Json(ShowUserRes {
        _id:            id,
        name:           "String".to_string(),
        passhash:       "String".to_string(),
        device_ids:     vec![],
    })
}
async fn update_user(Path(id): Path<ObjectId>) -> Json<UpdateUserRes> {
    Json(UpdateUserRes {user_id: id})
}

async fn list_cells() {}
async fn create_cell() {}
async fn show_cell(Path(id): Path<ObjectId>) -> Json<ShowCellRes> {
    Json(ShowCellRes {
        _id:            id,
        user_id:        id,
        device_id:      id,
        dir_ids:        vec![],
        fileprop_ids:   vec![],
        ancestor_ids:   vec![],
        text:           "String".to_string(),
        is_open:        true,
    })
}
async fn update_cell(Path(id): Path<ObjectId>) -> Json<UpdateCellRes> {
    Json(UpdateCellRes {
        cell_id: id,
    })
}


/*
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

    let broadcast_incoming = incoming.for_each(|msg_result| async {

        let msg = msg_result.unwrap();
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
      
        // メッセージを解析し、ファイルとコメントに分割する
        let data = msg.into_data();
        let (cmd_binary, body) = &data.split_at(1);
        let cmd = cmd_binary[0] as u64;
        let mut response: String = "".to_string();
        match cmd {
            2 =>  {
                let req: Cell = serde_json::from_slice(body).unwrap();
                let cells = db.collection("cells");
                let cell_id = cells.insert_one(req, None).await.unwrap().inserted_id;
                let cell_id = cell_id.as_object_id().unwrap();
                println!("Cell created with ID: {}", &cell_id);
                response = serde_json::to_string(&CreateCellRes{cell_id}).unwrap();
            }
            3 => {
                let req: Directory = serde_json::from_slice(body).unwrap();
                let dirs = db.collection("dirs");
                let parent_id_wrap = req.parent_id;
                let dir_id = dirs.insert_one(req, None).await.unwrap().inserted_id;
                let dir_id = dir_id.as_object_id().unwrap();
                println!("Dir created with ID: {}", &dir_id);
                if let Some(parent_id) = parent_id_wrap {
                    dirs.update_one(
                        doc! { "_id": parent_id },
                        doc! { "$push": {"dirs": dir_id} },
                        None,
                    ).await.unwrap();
                }
                response = serde_json::to_string(&CreateDirRes{dir_id}).unwrap();
            }
            4 => {
                let req: FileProp = serde_json::from_slice(body).unwrap();
                let file_props = db.collection("file_props");
                let parent_id_wrap = req.parent_id;
                let file_prop_id = file_props.insert_one(req, None).await.unwrap().inserted_id;
                let file_prop_id = file_prop_id.as_object_id().unwrap();
                println!("FileProp created with ID: {}", &file_prop_id);
                if let Some(parent_id) = parent_id_wrap {
                    file_props.update_one(
                        doc! { "_id": parent_id },
                        doc! { "$push": {"files": file_prop_id} },
                        None,
                    ).await.unwrap();
                }
                response = serde_json::to_string(&CreateFilePropRes{file_prop_id}).unwrap();
            }
            5 => {
                let req: FileBlob = serde_json::from_slice(body).unwrap();
                let bucket = db.gridfs_bucket(None);
                let blob = req.blob;
                let mut upload_stream = bucket.open_upload_stream(req.file_name, None);
                upload_stream.write_all(&blob[..]).await.unwrap();
                let uploaded_id = upload_stream.id();
                let uploaded_id = uploaded_id.as_object_id().unwrap();
                upload_stream.close().await.unwrap();
                println!("File uploaded with ID: {}", &uploaded_id);
                response = serde_json::to_string(&UploadFileRes{uploaded_id}).unwrap();
            }
            6 => {
                let req: UpdateCellReq = serde_json::from_slice(body).unwrap();
                let cells: Collection<Cell> = db.collection("cells");
                let mut update_doc = doc! {};
                if let Some(user_id) = req.user_id {
                    update_doc.insert("user_id", user_id);
                }
                if let Some(device_id) = req.device_id {
                    update_doc.insert("device_id", device_id);
                }
                if let Some(dir_ids) = req.dir_ids {
                    update_doc.insert("dir_ids", dir_ids);
                }
                if let Some(fileprop_ids) = req.fileprop_ids {
                    update_doc.insert("fileprop_ids", fileprop_ids);
                }
                if let Some(ancestor_ids) = req.ancestor_ids {
                    update_doc.insert("ancestor_ids", ancestor_ids);
                }
                if let Some(text) = req.text {
                    update_doc.insert("text", text);
                }
                if let Some(is_open) = req.is_open {
                    update_doc.insert("is_open", is_open);
                }
                // idで指定されたドキュメントを更新します
                let options = UpdateOptions::builder().upsert(false).build();
                cells.update_one(
                    doc! { "_id": req._id },
                    doc! { "$set": update_doc },
                    Some(options),
                ).await.unwrap();
            }
            7 => {
                let req: GetCellPropReq = serde_json::from_slice(body).unwrap();
                let cells: Collection<Cell> = db.collection("cells");

                let pipeline: Vec<mongodb::bson::Document> = vec![
                    doc! {
                        "$match": {
                            "_id": req.cell_id,
                        }
                    },
                    doc! {
                        "$graphLookup": {
                            "from": "devices",
                            "startWith": "$device_id",
                            "connectFromField": "device_id",
                            "connectToField": "_id",
                            "as": "device",
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
                    },
                    doc! {
                        "$graphLookup": {
                            "from": "fileprops",
                            "startWith": "$fileprop_ids",
                            "connectFromField": "fileprop_ids",
                            "connectToField": "_id",
                            "as": "fileprops"
                        }
            
                    },
                    doc! {
                        "$project": {
                            "user.passhash": 0,
                        }
                    },
                ];

                // 集計オプションを設定する
                let options = AggregateOptions::builder().build();
                let mut cursor = cells.aggregate(pipeline, options).await.unwrap();

                // 結果を処理する
                while let Some(result) = cursor.next().await {
                    match result {
                        Ok(document) => {
                            // ドキュメントを処理する
                            println!("{:?}", document);
                        }
                        Err(e) => {
                            // エラーを処理する
                            eprintln!("Error: {}", e);
                        }
                    }
                }

            }
            _ => {}
        }

        let peers = peer_map.lock().unwrap();
        let broadcast_recipients = peers.iter().map(|(_, ws_sink)| ws_sink);
        for recp in broadcast_recipients {
            recp.unbounded_send(response.clone().into()).unwrap();
        }

    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}
*/
