use log::error;
use crate::model::*;
use axum::debug_handler;
use axum::{
    extract::{Path, State},
    response::Json,
    http::StatusCode,
};

use sqlx::Postgres;
use sqlx::pool::Pool;

#[debug_handler]
pub async fn create_cell(
    Path(user_id): Path<uuid::Uuid>,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CellReq>
) -> Result<(), StatusCode> {
    
    let _id = payload.cell_id;
    let text = payload.text;
    let device_id = payload.device_id;
    let is_open = payload.is_open;

    if let Err(e) = sqlx::query(
      "INSERT INTO cells (device_id, text, is_open) 
      VALUES ($1, $2, $3)"
    )
        .bind(user_id)
        .bind(text)
        .bind(device_id)
        .bind(is_open)
        .execute(&pool)
        .await {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        };

    // Prepare the query
    let mut query = "INSERT INTO family_tree (child_id, parent_id) VALUES ".to_owned();

    // Prepare the values placeholder string and parameter values
    let parent_ids: Vec<uuid::Uuid> = payload.parent_ids;
    let mut params = Vec::new();
    for (i, parent_id) in parent_ids.into_iter().enumerate() {
        if i > 0 {
            query.push_str(", ");
        }
        query.push_str(&format!("(${}, ${})", i * 2 + 1, i * 2 + 2));
        params.push(_id);
        params.push(parent_id);
    }

    // Prepare the values placeholder string and parameter values
    let child_ids: Vec<uuid::Uuid> = payload.child_ids;
    let mut params = Vec::new();
    for (i, child_id) in child_ids.into_iter().enumerate() {
        if i > 0 {
            query.push_str(", ");
        }
        query.push_str(&format!("(${}, ${})", i * 2 + 1, i * 2 + 2));
        params.push(child_id);
        params.push(_id);
    }

    // Execute the query with parameters
    if let Err(e) = sqlx::query(&query)
        .bind(params)
        .execute(&pool)
        .await {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        };

    Ok(())
}

#[debug_handler]
pub async fn show_cell(
    Path(cell_id): Path<uuid::Uuid>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<CellExtracted>, StatusCode> {
    match sqlx::query_as("SELECT id, user_name, active FROM users WHERE id=$1")
        .bind(cell_id)
        .fetch_one(&pool)
        .await {
            Ok(cell) => Ok(Json(cell)),
            Err(e) => {
                error!("{}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
}

pub async fn delete_cell(
    Path((user_id, cell_id)): Path<(uuid::Uuid, uuid::Uuid)>,
    State(pool): State<Pool<Postgres>>,
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


/*
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
*/