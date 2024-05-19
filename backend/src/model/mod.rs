// use std::cell::{self, Cell};
use serde::{Deserialize, Serialize};
// use mongodb::bson::{doc, oid::ObjectId};
use sqlx::FromRow;

// pub mod request;
// pub use request::*;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct IdRes { pub id: uuid::Uuid }

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Users { pub users: Vec<User> }


#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id:        uuid::Uuid,
    pub user_name:      String,
    pub passhash:       String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Cells {
    pub cells:          Vec<CellExtracted>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CellReq {
    pub cell_id:        uuid::Uuid,
    pub user_id:        uuid::Uuid,
    pub device_id:      String,
    pub text:           String,
    pub fileprops:      sqlx::types::Json<FileProp>,
    pub parent_ids:     Vec<uuid::Uuid>,
    pub child_ids:      Vec<uuid::Uuid>,
    pub is_open:        bool,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CellExtracted {
    pub cell_id:        uuid::Uuid,
    pub user_id:        uuid::Uuid,
    pub device_id:      String,
    pub fileprops:      Vec<FileProp>,
    pub parents:        Vec<CellExtracted>,
    pub children:       Vec<CellExtracted>,
    pub text:           String,
    pub is_open:        bool,
}

impl CellExtracted {
    pub fn from_cell_row(
        cell_row: CellRow,
        parent_cells: Vec<CellExtracted>,
        child_cells: Vec<CellExtracted>,
    ) -> CellExtracted {
        CellExtracted {
            cell_id: cell_row.cell_id,
            user_id: cell_row.user_id,
            device_id: cell_row.device_id,
            text: cell_row.text,
            is_open: cell_row.is_open,
            fileprops: cell_row.fileprops.into_iter().map(|x|x.0).collect(),
            parents: parent_cells,
            children: child_cells,
        }
    }
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CellRow {
    pub cell_id:        uuid::Uuid,
    pub user_id:        uuid::Uuid,
    pub fileprops:      Vec<sqlx::types::Json<FileProp>>,
    pub text:           String,
    pub device_id:      String,
    pub is_open:        bool,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct FileProp {
    pub fileprop_id:    uuid::Uuid,
    pub name:           String,
    pub path:           String,
    pub file_url:       String,
    pub completed:      bool,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Parent {
    pub parent_id:    uuid::Uuid,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Child {
    pub child_id:    uuid::Uuid,
}