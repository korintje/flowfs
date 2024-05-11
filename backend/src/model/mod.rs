// use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};

pub mod request;
pub use request::*;

pub mod response;
pub use response::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Res {
    Error(anyhow::Error),
    User(User),
    Cell(Cell),
    Ok,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdRes { pub _id: ObjectId }


#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub _id:            [u8; 6],
    pub name:           String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id:            Option<ObjectId>,
    pub name:           String,
    pub passhash:       String,
    pub device_ids:     Vec<[u8;6]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id:            Option<ObjectId>,
    pub user_id:        ObjectId,
    pub device_id:      ObjectId,
    pub dir_ids:        Vec<ObjectId>,
    pub fileprop_ids:   Vec<ObjectId>,
    pub ancestor_ids:   Vec<ObjectId>,
    pub text:           String,
    pub is_open:        bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id:            Option<ObjectId>,
    pub user_id:        ObjectId,
    pub name:           String,
    pub dir_ids:        Vec<ObjectId>,
    pub fileprop_ids:   Vec<ObjectId>,
    pub parent_id:      Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileProp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id:            Option<ObjectId>,
    pub user_id:        ObjectId,
    pub name:           String,
    pub blob_id:        ObjectId,
    pub completed:      bool,
    pub parent_id:      Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBlob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id:            Option<ObjectId>,
    pub user_id:        ObjectId,
    pub file_name:      String,
    pub blob:           Vec<u8>,
}
