use log::error;
use mongodb::bson::{doc, Document}; 
use mongodb::bson::{oid::ObjectId, from_document};
use mongodb::options::{UpdateOptions, AggregateOptions};
use mongodb::{Collection, Database};
use futures_util::{StreamExt, TryStreamExt};

use crate::model::*;

use axum::debug_handler;
use axum::{
    extract::{Path, State},
    response::Json,
    http::StatusCode,
};

/*
#[debug_handler]
pub async fn list_cells(
    State(users): State<Collection<User>>,
    Path((user_id, cell_id)): Path<(ObjectId, ObjectId)>,
) -> Result<Json<User>, StatusCode> {
    let user = match users.find_one(doc!{"_id": user_id,}, None).await {
        Ok(user) => cursor,
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    let vd: Vec<Document> = cursor.try_collect().await.unwrap();
    
    // let vc: Vec<Cell> = vd.into_iter().map(|d| {from_document::<Cell>(d).unwrap()}).collect();
    // Ok(Json(User{cells: vc}))
}
*/


#[debug_handler]
pub async fn create_cell(
    Path(user_id): Path<ObjectId>,
    State(users): State<Collection<User>>,
    Json(payload): Json<Cell>
) -> Result<Json<Cell>, StatusCode> {
    // let cells: Collection<Cell> = db.collection("cells");
    let filter = doc! {"_id": user_id};
    let update = Document(doc! {"$push": {"cells": payload}});
    let options = mongodb::options::FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
        .build();
    match users.find_one_and_update(filter, update, options).await {
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
    let options = mongodb::options::FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
        .build();
    let update = doc! {
        "$set": {
            "cells.$.ancestor_ids": payload.ancestor_ids,
            "cells.$.text": payload.text,
            "cells.$.is_open": payload.is_open,
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
    let options = mongodb::options::FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
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