use log::error;
use mongodb::bson::{
    doc,
    oid::ObjectId,
    from_document
};
use mongodb::options::{
    AggregateOptions,
    FindOneAndUpdateOptions,
    ReturnDocument,
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

use sqlx::pool;

/*
#[debug_handler]
pub async fn create_cell(
    Path(user_id): Path<ObjectId>,
    State(users): State<Collection<User>>,
    Json(payload): Json<Cell>
) -> Result<Json<User>, StatusCode> {
    let filter = doc! {"_id": user_id};
    let update = doc! {"$push": {"cells": payload}};
    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();
    match users.find_one_and_update(filter, update, options).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(None) => {
                error!("Cell not found");
                Err(StatusCode::NOT_FOUND)
            },
        Ok(Some(user)) => Ok(Json(user))
    }
}
*/

#[debug_handler]
pub async fn create_cell(
    Path(user_id): Path<i64>,
    State(pool): State<sqlx::pool::Pool<sqlx::Postgres>>,
    Json(payload): Json<Cell>
) -> Result<Json<User>, StatusCode> {
    let ra: Result<Cell, sqlx::Error> = sqlx::query_as("SELECT id, user_name, active FROM users WHERE id=$1")
      .bind(user_id)
      .fetch_one(&pool)
      .await;
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

#[debug_handler]
pub async fn show_cell(
    Path((user_id, cell_id)): Path<(ObjectId, ObjectId)>,
    State(users): State<Collection<User>>
) -> Result<Json<Cell>, StatusCode> {
    let pipeline: Vec<mongodb::bson::Document> = vec![
        doc! {"$match": {"_id": user_id}},
        doc! {"$unwind": "$cells"},
        doc! {"$match": {"cells._id": cell_id}},
        doc! {"$limit": 1},
    ];
    let options = AggregateOptions::builder().build();
    let mut cursor = match users.aggregate(pipeline, options).await {
        Ok(cursor) => cursor,
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    match cursor.try_next().await {
        Ok(Some(cell)) => Ok(Json(from_document(cell).unwrap())),
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
pub async fn update_cell(
    Path((user_id, cell_id)): Path<(ObjectId, ObjectId)>,
    State(users): State<Collection<User>>,
    Json(payload): Json<UpdateCellReq>
) -> Result<Json<User>, StatusCode> {
    let filter = doc! {"_id": user_id, "cells._id": cell_id};
    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();
    let update = doc! {
        "$set": {
            "cells.$.ancestor_ids": payload.ancestor_ids.unwrap(),
            "cells.$.text": payload.text.unwrap(),
            "cells.$.is_open": payload.is_open.unwrap(),
        }
    };
    match users.find_one_and_update(filter, update, options).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(ou) => {
            match ou {
                None => {
                    error!("Cell not found.");
                    Err(StatusCode::NOT_FOUND)
                },
                Some(user) => Ok(Json(user))
            }
        },
    }
}

pub async fn delete_cell(
    Path((user_id, cell_id)): Path<(ObjectId, ObjectId)>,
    State(users): State<Collection<User>>
) -> Result<Json<User>, StatusCode> {
    let filter = doc! {"_id": user_id};
    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();
    let update = doc! {"$pull": {"cells": {"_id": cell_id}}};
    match users.find_one_and_update(filter, update, options).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(ou) => {
            match ou {
                None => {
                    error!("Cell not found.");
                    Err(StatusCode::NOT_FOUND)
                },
                Some(user) => Ok(Json(user))
            }
        },
    }
}