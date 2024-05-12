use std::env;
use env_logger;
use mongodb::Collection;

mod utils;
mod model;
mod handler;
use model::User;
use handler::{
    user::{list_users, create_user, show_user, update_user, delete_user},
    cell::{create_cell, show_cell, update_cell, delete_cell},
    // dir::{list_dirs, create_dir, show_dir, update_dir, delete_dir},
};

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {

    // Initialize logger
    env_logger::init();

    // Set web URL
    let listen_url = utils::get_url();
    let addr = env::args().nth(1).unwrap_or_else(|| listen_url);

    // MongoDBクライアントのセットアップ
    // Ref: https://www.mongodb.com/docs/drivers/rust/current/quick-start/connect-to-mongodb/
    let db_url = utils::get_db_path();
    let db_client = mongodb::Client::with_uri_str(db_url).await.unwrap();
    let db = db_client.database("flowfs");
    let col: Collection<User> = db.collection("users");

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:user_id", get(show_user).put(update_user).delete(delete_user))
        .route("/users/:user_id/cells/", post(create_cell))
        .route("/users/:user_id/cells/:cell_id", get(show_cell).put(update_cell).delete(delete_cell))
        .with_state(col);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}



/*

        match cmd {

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
            7 => { }

*/
