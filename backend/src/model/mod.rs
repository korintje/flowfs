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
    pub device_id:      String,
    pub fileprops:      Vec<uuid::Uuid>,
    pub parent_ids:     Vec<uuid::Uuid>,
    pub child_ids:      Vec<uuid::Uuid>,
    pub text:           String,
    pub is_open:        bool,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CellExtracted {
    pub cell_id:        uuid::Uuid,
    pub fileprops:      Vec<FileProp>,
    pub parents:        Vec<CellExtracted>,
    pub children:       Vec<CellExtracted>,
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
