use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;


// User
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserReq {
    pub name:           String,
    pub passhash:       String,
    pub device_ids:     Vec<[u8;6]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserReq {
    pub user_id:        ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserReq {
    pub name:           Option<String>,
    pub passhash:       Option<String>,
    pub device_ids:     Option<Vec<[u8;6]>>,
}

// Cell
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCellReq {
    pub user_id:        ObjectId,
    pub device_id:      ObjectId,
    pub dir_ids:        Vec<ObjectId>,
    pub fileprop_ids:   Vec<ObjectId>,
    pub ancestor_ids:   Vec<ObjectId>,
    pub text:           String,
    pub is_open:        bool,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDirReq {
    pub user_id:        Option<ObjectId>,
    pub name:           Option<String>,
    pub dir_ids:        Option<Vec<ObjectId>>,
    pub fileprop_ids:   Option<Vec<ObjectId>>,
    pub parent_id:      Option<ObjectId>,
}