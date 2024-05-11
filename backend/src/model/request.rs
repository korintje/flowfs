use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserReq {
    pub user_id:        ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserReq {
    pub user_id:        ObjectId,
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