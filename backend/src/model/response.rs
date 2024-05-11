use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRes {
    pub user_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRes {
    pub user_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowUserRes {
    pub _id:            ObjectId,
    pub name:           String,
    pub passhash:       String,
    pub device_ids:     Vec<[u8;6]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceRes {
    pub device_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCellRes {
    pub cell_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCellRes {
    pub cell_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowCellRes {
    pub _id:            ObjectId,
    pub user_id:        ObjectId,
    pub device_id:      ObjectId,
    pub dir_ids:        Vec<ObjectId>,
    pub fileprop_ids:   Vec<ObjectId>,
    pub ancestor_ids:   Vec<ObjectId>,
    pub text:           String,
    pub is_open:        bool,
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