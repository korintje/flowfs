use serde::{Deserialize, Serialize};
// use mongodb::bson::oid::ObjectId;


// User
/*
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



*/