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

use async_recursion::async_recursion;

#[debug_handler]
pub async fn create_cell(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CellReq>
) -> Result<(), StatusCode> {
    let cell_id = payload.cell_id;
    let parent_ids = payload.parent_ids;
    let child_ids = payload.child_ids;
    if let Err(e) = sqlx::query(
      "INSERT INTO cells (cell_id, user_id, device_id, text, fileprops, is_open) 
      VALUES ($1, $2, $3, $4, $5, $6)"
    )
        .bind(cell_id)
        .bind(payload.user_id)
        .bind(payload.device_id)
        .bind(payload.text)
        .bind(payload.fileprops)
        .bind(payload.is_open)
        .execute(&pool)
        .await {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        };

    // Prepare the query
    let mut query = "INSERT INTO family_tree (child_id, parent_id) VALUES ".to_owned();
    let mut params = Vec::new();
    for (i, parent_id) in parent_ids.into_iter().enumerate() {
        if i > 0 {
            query.push_str(", ");
        }
        query.push_str(&format!("(${}, ${})", i * 2 + 1, i * 2 + 2));
        params.push(cell_id);
        params.push(parent_id);
    }
    for (i, child_id) in child_ids.into_iter().enumerate() {
        if i > 0 {
            query.push_str(", ");
        }
        query.push_str(&format!("(${}, ${})", i * 2 + 1, i * 2 + 2));
        params.push(child_id);
        params.push(cell_id);
    }
    if let Err(e) = sqlx::query(&query)
        .bind(params)
        .execute(&pool)
        .await {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        };

    Ok(())
}

async fn get_cell(
    cell_id: &uuid::Uuid,
    pool: &Pool<Postgres>,
) -> Result<CellRow, sqlx::Error> {
    sqlx::query_as(
        "SELECT cell_id, user_id, device_id, text, fileprops, is_open, created_at 
        FROM cells WHERE cell_id=$1"
    )
    .bind(cell_id)
    .fetch_one(pool)
    .await
}

async fn get_child_ids(
    cell_id: &uuid::Uuid,
    pool: &Pool<Postgres>,  
) -> Result<Vec<uuid::Uuid>, sqlx::Error> {
    let child_ids: Vec<Child> = sqlx::query_as(
        "SELECT child_id FROM family_tree WHERE paret_id=$1"
    ).bind(cell_id).fetch_all(pool).await?;
    let child_ids = child_ids.into_iter().map(|row: Child| row.child_id).collect();
    Ok(child_ids)
}

async fn get_parent_ids(
    cell_id: &uuid::Uuid,
    pool: &Pool<Postgres>,  
) -> Result<Vec<uuid::Uuid>, sqlx::Error> {
    let parent_ids: Vec<Parent> = sqlx::query_as(
        "SELECT parent_id FROM family_tree WHERE paret_id=$1"
    ).bind(cell_id).fetch_all(pool).await?;
    let parent_ids = parent_ids.into_iter().map(|row: Parent| row.parent_id).collect();
    Ok(parent_ids)
}

#[async_recursion]
async fn make_child_tree(
    cell_id: &uuid::Uuid,
    pool: &Pool<Postgres>,
) -> Option<CellExtracted> {
    let Ok(cell) = get_cell(cell_id, pool).await else {
        return None
    };
    let child_ids = get_child_ids(cell_id, pool).await.unwrap_or(vec![]);
    let mut cell_ext = CellExtracted::from_cell_row(
        cell, vec![], vec![],
    );
    for child_id in child_ids.iter() {
        if let Some(c) = make_child_tree(child_id, pool).await {
            cell_ext.children.push(c);
        }
    }
    Some(cell_ext)
}

#[async_recursion]
async fn make_parent_tree(
    cell_id: &uuid::Uuid,
    pool: &Pool<Postgres>,
) -> Option<CellExtracted> {
    let Ok(cell) = get_cell(cell_id, pool).await else {
        return None
    };
    let parent_ids = get_parent_ids(cell_id, pool).await.unwrap_or(vec![]);
    let mut cell_ext = CellExtracted::from_cell_row(
        cell, vec![], vec![],
    );
    for parent_id in parent_ids.iter() {
        if let Some(c) = make_parent_tree(parent_id, pool).await {
            cell_ext.parents.push(c);
        }
    }
    Some(cell_ext)
}

#[debug_handler]
pub async fn show_cell(
    Path(cell_id): Path<uuid::Uuid>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<CellExtracted>, StatusCode> {
    let Some(mut cell_with_children) = make_child_tree(&cell_id, &pool).await else {
        return Err(StatusCode::NOT_FOUND)
    };
    let Some(cell_with_parents) = make_parent_tree(&cell_id, &pool).await else {
        return Err(StatusCode::NOT_FOUND)
    };
    cell_with_children.parents = cell_with_parents.parents;
    Ok(Json(cell_with_children))
}

pub async fn delete_cell(
    Path(cell_id): Path<uuid::Uuid>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<IdRes>, StatusCode> {
    match sqlx::query("DELETE FROM cells WHERE id=$1")
    .bind(cell_id)
    .execute(&pool)
    .await {
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::NOT_FOUND)
        }
        Ok(_) => Ok(Json(IdRes{id: cell_id})),
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