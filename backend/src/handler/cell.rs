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

#[debug_handler]
pub async fn list_cells(State(db): State<Database>) -> Result<Json<CellsRes>, StatusCode> {
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
pub async fn create_cell(
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
pub async fn show_cell(
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
                "from": "dirs",
                "startWith": "$dir_ids",
                "connectFromField": "dir_ids",
                "connectToField": "_id",
                "as": "dirs"
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
pub async fn update_cell(
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

pub async fn delete_cell(
    Path(id): Path<ObjectId>,
    State(db): State<Database>
) -> Result<Json<IdRes>, StatusCode> {
    let cells: Collection<Dir> = db.collection("cells");
    match cells.delete_one(doc! {"_id": id}, None).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(_r) => Ok(Json(IdRes{_id: id}))
    }
}