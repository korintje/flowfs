use log::error;
use mongodb::bson::{
    doc,
    oid::ObjectId
};
use mongodb::options::{
    ReturnDocument,
    FindOneOptions,
    FindOneAndUpdateOptions
};
use mongodb::Collection;
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
    State(users): State<Collection<User>>, 
) -> Result<Json<Users>, StatusCode> {
    let cursor = match users.find(doc!{}, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    let users: Vec<User> = cursor.try_collect().await.unwrap();
    Ok(Json(Users{users}))
}

#[debug_handler]
pub async fn create_user(
    State(users): State<Collection<User>>, 
    Json(payload): Json<User>
) -> Result<Json<IdRes>, StatusCode> {
    match users.insert_one(&payload, None).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(rt) => {
            match rt.inserted_id.as_object_id() {
                None => {
                    error!("ID not found");
                    Err(StatusCode::NOT_FOUND)             
                },
                Some(_id) => Ok(Json(IdRes{_id}))
            }
        }
    }
}


#[debug_handler]
pub async fn show_user(
    Path(id): Path<ObjectId>,
    State(users): State<Collection<User>>, 
) -> Result<Json<User>, StatusCode> {
    let opts = FindOneOptions::builder()
        .projection(doc!{"cells": 0})
        .build();
    match  users.find_one(doc!{"_id": id}, opts).await {
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
    State(users): State<Collection<User>>,
    Json(payload): Json<UpdateUserReq>, 
) -> Result<Json<User>, StatusCode> {
    let filter = doc! {"_id": id};
    let opts = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();
    let mut update = doc! {};
    if let Some(name) = payload.name {
        update.insert("$set", doc!{"name": name});
    }
    if let Some(passhash) = payload.passhash {
        update.insert("$set", doc!{"passhash": passhash});
    }
    match users.find_one_and_update(filter, update, opts).await {
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(None) => {
            error!("{}", "User not found");
            return Err(StatusCode::NOT_FOUND)
        },
        Ok(Some(user)) => {
            Ok(Json(user))
        }
    }
}

#[debug_handler]
pub async fn delete_user(
    Path(id): Path<ObjectId>,
    State(users): State<Collection<User>>,
) -> Result<Json<IdRes>, StatusCode> {
    match users.delete_one(doc! {"_id": id}, None).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(_r) => Ok(Json(IdRes{_id: id}))
    }
}
