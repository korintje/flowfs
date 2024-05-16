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

use sqlx::Postgres;
use sqlx::pool::Pool;

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
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<Cell>
) -> Result<(), StatusCode> {
    
    let _id = payload._id;
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
    let fileprops: Vec<FileProp> = payload.fileprops;
    let mut query = "INSERT INTO fileprops (name, file_url, completed) VALUES ".to_owned();
  
    // Prepare the values placeholder string and parameter values
    let mut params = Vec::new();
    for (i, fileprop) in fileprops.iter().enumerate() {
        if i > 0 {
            query.push_str(", ");
        }
        query.push_str(&format!("(${}, ${}, ${})", i * 3 + 1, i * 3 + 2, i * 3 + 3));
        params.push(fileprop.name.as_str());
        params.push(fileprop.file_url.as_str());
        params.push(if fileprop.completed {"1"} else {"0"});
    }

    // Execute the query with parameters
    if let Err(e) = sqlx::query(&query)
        .bind(params)
        .execute(&pool)
        .await {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        };

    // Prepare the query
    let ancestor_ids: Vec<i64> = payload.ancestor_ids;
    let mut query = "INSERT INTO ancestor_ids (descendant_id, ancestor_id) VALUES ".to_owned();

    // Prepare the values placeholder string and parameter values
    let mut params = Vec::new();
    for (i, ancestor_id) in ancestor_ids.into_iter().enumerate() {
        if i > 0 {
            query.push_str(", ");
        }
        query.push_str(&format!("(${}, ${})", i * 2 + 1, i * 2 + 2));
        params.push(_id);
        params.push(ancestor_id);
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
    Path((user_id, cell_id)): Path<(i64, i64)>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Cells>, StatusCode> {
    sqlx::query_as(
        "SELECT cell_id, resource_id, resource_name, start, end, description, passhash, created_at 
        FROM fileprops JOIN users ON (fileprops.user_id=users.user_id) JOIN cells ON (fileprops.cell_id=cells.cell_id) 
        WHERE (cell_id=$1)"
      )
      .bind(cell_id)
      .fetch_one(&pool)
      .await
      .map(|obj: Cell| Cell::from(obj))

}

/*
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
*/

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