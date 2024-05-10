// use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};

pub mod request;
pub use request::*;

pub mod response;
pub use response::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub _id:            [u8; 6],
    pub name:           String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id:            ObjectId,
    pub name:           String,
    pub passhash:       String,
    pub device_ids:     Vec<[u8;6]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    pub _id:            ObjectId,
    pub user_id:        ObjectId,
    pub device_id:      ObjectId,
    pub dir_ids:        Vec<ObjectId>,
    pub fileprop_ids:   Vec<ObjectId>,
    pub ancestor_ids:   Vec<ObjectId>,
    pub text:           String,
    pub open_with:      Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    pub _id:            ObjectId,
    pub user_id:        ObjectId,
    pub name:           String,
    pub dir_ids:        Vec<ObjectId>,
    pub fileprop_ids:   Vec<ObjectId>,
    pub parent_id:      Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileProp {
    pub _id:            ObjectId,
    pub user_id:        ObjectId,
    pub name:           String,
    pub blob_id:        ObjectId,
    pub completed:      bool,
    pub parent_id:      Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBlob {
    pub _id:            ObjectId,
    pub user_id:        ObjectId,
    pub file_name:      String,
    pub blob:           Vec<u8>,
}
