use log::error;
use mongodb::bson::{doc, Document}; 
use mongodb::bson::{oid::ObjectId, from_document};
use mongodb::options::{UpdateOptions, AggregateOptions};
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
pub async fn list_dirs(
    State(db): State<Database>, 
) -> Result<Json<DirsRes>, StatusCode> {
    let dirs: Collection<Dir> = db.collection("dirs");
    let pipeline: Vec<mongodb::bson::Document> = vec![
        doc! {
            "$match": {}
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
    let cursor = match dirs.aggregate(pipeline, options).await {
        Ok(cursor) => cursor,
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    let vd: Vec<Document> = cursor.try_collect().await.unwrap();
    let vc: Vec<DirRes> = vd.into_iter().map(|d| {from_document::<DirRes>(d).unwrap()}).collect();
    Ok(Json(DirsRes{dirs: vc}))
}


#[debug_handler]
pub async fn create_dir(
    State(db): State<Database>, 
    Json(payload): Json<Dir>
) -> Result<Json<Dir>, StatusCode> {
    let parent_id_wrap = payload.parent_id;
    let dirs: Collection<Dir> = db.collection("dirs");
    let rt = match dirs.insert_one(&payload, None).await {
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(rt) => rt
    };
    let mut inserted_id = rt.inserted_id; 
    if let Some(parent_id) = parent_id_wrap {
        match dirs.update_one(
            doc! { "_id": parent_id },
            doc! { "$push": {"dirs": inserted_id} },
            None,
        ).await {
            Err(e) => {
                error!("{}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Ok(rt2) => inserted_id = rt2.upserted_id.unwrap()
        }
    };
    match dirs.find_one(doc!{"_id": inserted_id}, None).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(None) => {
            error!("Dir not found");
            Err(StatusCode::NOT_FOUND)            
        }
        Ok(Some(dir)) => {
            
            Ok(Json(dir))
        }
    }
}


#[debug_handler]
pub async fn show_dir(
    Path(id): Path<ObjectId>,
    State(db): State<Database>
) -> Result<Json<Dir>, StatusCode> {
    let dirs: Collection<Dir> = db.collection("dirs");
    let pipeline: Vec<mongodb::bson::Document> = vec![
        doc! {
            "$match": {
                "_id": id,
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
    let mut cursor = match dirs.aggregate(pipeline, options).await {
        Ok(cursor) => cursor,
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    match cursor.try_next().await {
        Ok(Some(dir_res)) => Ok(Json(from_document(dir_res).unwrap())),
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
pub async fn update_dir(
    Path(id): Path<ObjectId>,
    State(db): State<Database>,
    Json(payload): Json<UpdateDirReq>, 
) -> Result<Json<Dir>, StatusCode> {
    let dirs: Collection<Dir> = db.collection("dirs");
    let mut update_doc = doc! {};
    /*
    if let Some(name) = payload.name {
        update_doc.insert("name", name);
    }
    if let Some(passhash) = payload.passhash {
        update_doc.insert("passhash", passhash);
    }
    if let Some(device_ids) = payload.device_ids {
        update_doc.insert("device_ids", device_ids);
    }
    */
    let options = UpdateOptions::builder().upsert(false).build();
    match dirs.update_one(
        doc! { "_id": id },
        doc! { "$set": update_doc },
        Some(options),
    ).await {
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(r) => {
            match dirs.find_one(doc!{"_id": r.upserted_id}, None).await {
                Err(e) => {
                    error!("{}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                },
                Ok(None) => {
                    error!("Dir not found");
                    Err(StatusCode::NOT_FOUND)
                },
                Ok(Some(dir)) => Ok(Json(dir))
            }
        },
    }
}

#[debug_handler]
pub async fn delete_dir(
    Path(id): Path<ObjectId>,
    State(db): State<Database>
) -> Result<Json<IdRes>, StatusCode> {
    let dirs: Collection<Dir> = db.collection("dirs");
    match dirs.delete_one(doc! {"_id": id}, None).await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(_r) => Ok(Json(IdRes{_id: id}))
    }
}

