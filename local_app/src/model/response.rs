use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRes {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceRes {
    pub device_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLumpRes {
    pub lump_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDirRes {
    pub dir_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFilePropRes {
    pub file_prop_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFileRes {
    pub uploaded_id: ObjectId,
}