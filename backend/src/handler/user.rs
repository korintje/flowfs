use log::error;
use mongodb::bson::doc; 
use mongodb::bson::oid::ObjectId;
use mongodb::options::UpdateOptions;
use mongodb::{Collection, Database};
use futures_util::TryStreamExt;

use crate::model::*;

use axum::debug_handler;
use axum::{
    extract::{Path, State},
    response::Json,
    http::StatusCode,
};

#[debug_handler]
pub async fn list_users(
    State(db): State<Database>, 
) -> Result<Json<UsersRes>, StatusCode> {
    let users: Collection<User> = db.collection("users");
    let cursor = match users.find(doc!{}, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    let vu: Vec<User> = cursor.try_collect().await.unwrap();
    Ok(Json(UsersRes{users: vu}))
}

#[debug_handler]
pub async fn create_user(
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
pub async fn show_user(
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
pub async fn update_user(
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
pub async fn delete_user(
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
