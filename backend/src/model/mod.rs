// use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};

pub mod request;
pub use request::*;

pub mod response;
pub use response::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub name:           String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub name:           String,
    pub passhash:       String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lump {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
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
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub user_id:        ObjectId,
    pub name:           String,
    pub dir_ids:        Vec<ObjectId>,
    pub fileprop_ids:   Vec<ObjectId>,   
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileProp {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub user_id:        ObjectId,
    pub name:           String,
    pub blob_id:        ObjectId,
    pub completed:      bool,
}