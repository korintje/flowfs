use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLumpRes {
    pub lump_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFileRes {
    pub uploaded_id: ObjectId,
}