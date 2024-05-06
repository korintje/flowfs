use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserReq {
    pub name:           String,
    pub passhash:       String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceReq {
    pub name:           String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLumpReq {
    pub user_id:        ObjectId,
    pub device_id:      ObjectId,
    pub dir_ids:        Vec<ObjectId>,
    pub fileprop_ids:   Vec<ObjectId>,
    pub ancestor_ids:   Vec<ObjectId>,
    pub text:           String,
    pub is_open:        bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDirReq {
    pub user_id:        ObjectId,
    pub name:           String,
    pub dir_ids:        Vec<ObjectId>,
    pub fileprop_ids:   Vec<ObjectId>,
    pub parent_id:      Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFilePropReq {
    pub user_id:        ObjectId,
    pub name:           String,
    pub blob_id:        ObjectId,
    pub completed:      bool,
    pub parent_id:      Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFileReq {
    pub user_id:        ObjectId,
    pub file_name:      String,
    pub blob:           Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLumpReq {
    pub id:             ObjectId,
    pub user_id:        Option<ObjectId>,
    pub device_id:      Option<ObjectId>,
    pub dir_ids:        Option<Vec<ObjectId>>,
    pub fileprop_ids:   Option<Vec<ObjectId>>,
    pub ancestor_ids:   Option<Vec<ObjectId>>,
    pub text:           Option<String>,
    pub is_open:        Option<bool>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetLumpPropReq {
    pub lump_id:        ObjectId,
}