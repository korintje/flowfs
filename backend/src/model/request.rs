use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLumpReq {
    pub user_id:        ObjectId,
    pub device_id:      mongodb::bson::Uuid,
    pub attachment_ids: Vec<ObjectId>,
    pub ancestor_ids:   Vec<ObjectId>,
    pub text:           String,
    pub is_open:        bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLumpPropReq {
    pub lump_id:        ObjectId,
}