use std::env;
use env_logger;
use log::error;
use mongodb::bson::{doc, Document}; 
use mongodb::bson::{oid::ObjectId, from_document};
use mongodb::options::{UpdateOptions, AggregateOptions};
use mongodb::{Collection, Database};
use futures_util::{StreamExt, TryStreamExt};

mod utils;
mod model;
use model::*;

use axum::debug_handler;
use axum::{
    routing::get,
    Router,
    extract::{Path, State},
    response::Json,
    http::StatusCode,
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
async fn create_user(
    State(db): State<Database>, 
    Json(payload): Json<User>
) -> Result<Json<User>, StatusCode> {
    let users: Collection<User> = db.collection("users");
    let rt = match users.insert_one(&payload, None).await {
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(rt) => rt
    };
    match users.find_one(doc!{"_id": rt.inserted_id}, None).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(None) => {
            error!("User not found");
            Err(StatusCode::NOT_FOUND)            
        }
        Ok(Some(user)) => Ok(Json(user))
    }
}


#[debug_handler]
async fn show_user(
    Path(id): Path<ObjectId>,
    State(db): State<Database>
) -> Result<Json<User>, StatusCode> {
    let users: Collection<User> = db.collection("users");
    match  users.find_one(doc!{"_id": id}, None).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => {
            error!("User not found");
            Err(StatusCode::NOT_FOUND)
        },
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[debug_handler]
async fn update_user(
    Path(id): Path<ObjectId>,
    State(db): State<Database>,
    Json(payload): Json<UpdateUserReq>, 
) -> Result<Json<User>, StatusCode> {
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
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(r) => {
            match users.find_one(doc!{"_id": r.upserted_id}, None).await {
                Err(e) => {
                    error!("{}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                },
                Ok(None) => {
                    error!("User not found");
                    Err(StatusCode::NOT_FOUND)
                },
                Ok(Some(user)) => Ok(Json(user))
            }
        },
    }
}

#[debug_handler]
async fn delete_user(
    Path(id): Path<ObjectId>,
    State(db): State<Database>
) -> Result<Json<IdRes>, StatusCode> {
    let users: Collection<User> = db.collection("users");
    match users.delete_one(doc! {"_id": id}, None).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(_r) => Ok(Json(IdRes{_id: id}))
    }
}


// Cell
#[debug_handler]
async fn list_cells(State(db): State<Database>) -> Result<Json<CellsRes>, StatusCode> {
    let cells: Collection<Cell> = db.collection("cells");
    let pipeline: Vec<mongodb::bson::Document> = vec![
        doc! {
            "$match": {}
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
    let options = AggregateOptions::builder().build();
    let cursor = match cells.aggregate(pipeline, options).await {
        Ok(cursor) => cursor,
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    let vd: Vec<Document> = cursor.try_collect().await.unwrap();
    let vc: Vec<CellRes> = vd.into_iter().map(|d| {from_document::<CellRes>(d).unwrap()}).collect();
    Ok(Json(CellsRes{cells: vc}))
}



#[debug_handler]
async fn create_cell(
    State(db): State<Database>,
    Json(payload): Json<Cell>
) -> Result<Json<Cell>, StatusCode> {
    let cells: Collection<Cell> = db.collection("cells");
    match cells.insert_one(&payload, None).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(r) => {
            match cells.find_one(doc!{"_id": r.inserted_id}, None).await {
                Err(e) => {
                    error!("{}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                },
                Ok(None) => {
                    error!("Cell not found");
                    Err(StatusCode::NOT_FOUND)
                },
                Ok(Some(cell)) => Ok(Json(cell))
            }
        },
    }
}


#[debug_handler]
async fn show_cell(
    Path(id): Path<ObjectId>,
    State(db): State<Database>
) -> Result<Json<CellRes>, StatusCode> {
    let cells: Collection<Cell> = db.collection("cells");
    let pipeline: Vec<mongodb::bson::Document> = vec![
        doc! {
            "$match": {
                "_id": id,
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
        doc! {
            "$limit": 1
        },
    ];
    let options = AggregateOptions::builder().build();
    let mut cursor = match cells.aggregate(pipeline, options).await {
        Ok(cursor) => cursor,
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    match cursor.try_next().await {
        Ok(Some(cell_res)) => Ok(Json(from_document(cell_res).unwrap())),
        Ok(None) => {
            error!("{}", "Cel not found");
            Err(StatusCode::NOT_FOUND)        
        },
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)  
        }
    }
}


#[debug_handler]
async fn update_cell(
    Path(id): Path<ObjectId>, 
    State(db): State<Database>,
    Json(payload): Json<UpdateCellReq>
) -> Result<Json<Cell>, StatusCode> {
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
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(r) => {
            match cells.find_one(doc!{"_id": r.upserted_id}, None).await {
                Err(e) => {
                    error!("{}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                },
                Ok(None) => {
                    error!("Cell not found.");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                },
                Ok(Some(cell)) => Ok(Json(cell))
            }
        },
    }
}

async fn delete_cell(
    Path(id): Path<ObjectId>,
    State(db): State<Database>
) -> Result<Json<IdRes>, StatusCode> {
    let cells: Collection<User> = db.collection("cells");
    match cells.delete_one(doc! {"_id": id}, None).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(_r) => Ok(Json(IdRes{_id: id}))
    }
}


// Directory



/*

        match cmd {
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
            7 => { }

*/
