// use sqlx::FromRow;
use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Serialize};
use mongodb::{Client, Collection};
use mongodb::bson::{doc, oid::ObjectId};

pub mod request;
pub use request::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub name:           String,
    pub created_at:     DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id:             ObjectId,
    pub name:           String,
    pub passhash:       String,
    pub created_at:     DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lump {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub user_id:        ObjectId,
    pub device_id:      mongodb::bson::Uuid,
    pub dir_ids:        Vec<ObjectId>,
    pub file_ids:       Vec<ObjectId>,
    pub ancestor_ids:   Vec<ObjectId>,
    pub text:           String,
    pub is_open:        bool,
    pub created_at:     DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub user_id:        ObjectId,
    pub name:           String,
    pub content_ids:    Vec<ObjectId>,
    pub created_at:     DateTime<Local>,    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub user_id:        ObjectId,
    pub name:           String,
    pub created_at:     DateTime<Local>,

}
