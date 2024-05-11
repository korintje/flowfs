use std::env;
use mongodb::bson::doc; 
use mongodb::bson::oid::ObjectId;
use mongodb::options::{UpdateOptions, AggregateOptions};
use mongodb::{Collection, Database};
use anyhow::anyhow;

mod utils;
mod model;
use model::*;

// type Tx = UnboundedSender<Message>;
// type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

use axum::debug_handler;
use axum::{
    routing::get,
    Router,
    extract::{Path, State},
    response::Json,
    http::StatusCode,
    response::IntoResponse,
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
    // let db_arc = std::sync::Arc::new(db);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(show_user).put(update_user).delete(delete_user))
        .route("/cells/", get(list_cells).post(create_cell))
        .route("/cells/:id", get(show_cell).put(update_cell).delete(delete_cell))
        .with_state(db);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

// User
async fn list_users() {}

#[debug_handler]
async fn create_user(State(db): State<Database>, Json(payload): Json<User>) -> anyhow::Result<Json<User>> {
    let users: Collection<User> = db.collection("users");
    let r = users.insert_one(&payload, None).await?;
    let ou = users.find_one(doc!{"_id": r.inserted_id}, None).await?;
    let user = ou.unwrap();
    Ok(Json(user))
}

/*
async fn create_user(Json(payload): Json<User>, State(db): State<Database>,) -> (StatusCode, Json<Res>) {
    let users: Collection<User> = db.collection("users");
    match users.insert_one(&payload, None).await {
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(e.into()))),
        Ok(r) => {
            match users.find_one(doc!{"_id": r.inserted_id}, None).await {
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(e.into()))),
                Ok(None) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(anyhow!("User not found")))),
                Ok(Some(user)) => return (StatusCode::CREATED, Json(Res::User(user)))
            }
        },
    }
}
*/

#[debug_handler]
async fn show_user(Path(id): Path<ObjectId>, State(db): State<Database>) -> (StatusCode, Json<Res>) {
    let users: Collection<User> = db.collection("users");
    let Ok(Some(user)) = users.find_one(doc!{"_id": id}, None).await else {
        return (StatusCode::NOT_FOUND, Json(Res::Error(anyhow!("User not found"))))
    };
    (StatusCode::OK, Json(Res::User(user)))
}

#[debug_handler]
async fn update_user(
    Path(id): Path<ObjectId>, 
    Json(payload): Json<UpdateUserReq>, 
    State(db): State<Database>
) -> (StatusCode, Json<Res>) {
    let users: Collection<User> = db.collection("users");
    let mut update_doc = doc! {};
    if let Some(name) = payload.name {
        update_doc.insert("name", name);
    }
    if let Some(passhash) = payload.passhash {
        update_doc.insert("passhash", passhash);
    }
    /*
    if let Some(device_ids) = payload.device_ids {
        update_doc.insert("device_ids", device_ids);
    }
    */
    let options = UpdateOptions::builder().upsert(false).build();
    match users.update_one(
        doc! { "_id": id },
        doc! { "$set": update_doc },
        Some(options),
    ).await {
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(e.into()))),
        Ok(r) => {
            match users.find_one(doc!{"_id": r.upserted_id}, None).await {
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(e.into()))),
                Ok(None) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(anyhow!("User not found")))),
                Ok(Some(user)) => return (StatusCode::OK, Json(Res::User(user)))
            }
        },
    }
}

#[debug_handler]
async fn delete_user(Path(id): Path<ObjectId>, State(db): State<Database>) -> (StatusCode, Json<Res>) {
    let users: Collection<User> = db.collection("users");
    match users.delete_one(doc! {"_id": id}, None).await {
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(e.into()))),
        Ok(_r) => (StatusCode::OK, Json(Res::Ok))
    }
}

// Cell
#[debug_handler]
async fn list_cells() {}

#[debug_handler]
async fn create_cell(Json(payload): Json<Cell>, State(db): State<Database>) -> (StatusCode, Json<Res>) {
    let cells: Collection<Cell> = db.collection("cells");
    match cells.insert_one(&payload, None).await {
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(e.into()))),
        Ok(r) => {
            match cells.find_one(doc!{"_id": r.inserted_id}, None).await {
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(e.into()))),
                Ok(None) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(anyhow!("Cell not found")))),
                Ok(Some(cell)) => return (StatusCode::CREATED, Json(Res::Cell(cell)))
            }
        },
    }
}

#[debug_handler]
async fn show_cell(Path(id): Path<ObjectId>, State(db): State<Database>) -> (StatusCode, Json<Res>) {
    let cells: Collection<Cell> = db.collection("cells");
    let Ok(Some(cell)) = cells.find_one(doc!{"_id": id}, None).await else {
        return (StatusCode::NOT_FOUND, Json(Res::Error(anyhow!("User not found"))))
    };
    (StatusCode::OK, Json(Res::Cell(cell)))
}

#[debug_handler]
async fn update_cell(
    Path(id): Path<ObjectId>, 
    Json(payload): Json<UpdateCellReq>, 
    State(db): State<Database>
) -> (StatusCode, Json<Res>) {
    let cells: Collection<Cell> = db.collection("cells");
    let mut update_doc = doc! {};
    /*
    if let Some(name) = payload.user_id {
        update_doc.insert("user_id", user_id);
    }
    if let Some(passhash) = payload.passhash {
        update_doc.insert("passhash", passhash);
    }
    if let Some(device_ids) = payload.device_ids {
        update_doc.insert("device_ids", device_ids);
    }
    */
    let options = UpdateOptions::builder().upsert(false).build();
    match cells.update_one(
        doc! { "_id": id },
        doc! { "$set": update_doc },
        Some(options),
    ).await {
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(e.into()))),
        Ok(r) => {
            match cells.find_one(doc!{"_id": r.upserted_id}, None).await {
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(e.into()))),
                Ok(None) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(anyhow!("Cell not found")))),
                Ok(Some(cell)) => return (StatusCode::OK, Json(Res::Cell(cell)))
            }
        },
    }
}

async fn delete_cell(Path(id): Path<ObjectId>, State(db): State<Database>) -> (StatusCode, Json<Res>) {
    let cells: Collection<User> = db.collection("cells");
    match cells.delete_one(doc! {"_id": id}, None).await {
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Res::Error(e.into()))),
        Ok(_r) => (StatusCode::OK, Json(Res::Ok))
    }
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
