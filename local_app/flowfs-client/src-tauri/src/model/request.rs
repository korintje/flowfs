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
pub struct UpdateCellReq {
    pub _id:            ObjectId,
    pub user_id:        Option<ObjectId>,
    pub device_id:      Option<ObjectId>,
    pub dir_ids:        Option<Vec<ObjectId>>,
    pub fileprop_ids:   Option<Vec<ObjectId>>,
    pub ancestor_ids:   Option<Vec<ObjectId>>,
    pub text:           Option<String>,
    pub is_open:        Option<bool>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetCellPropReq {
    pub cell_id:        ObjectId,
}