use specta::Type;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Type, Debug)]
pub struct IdRes { pub id: uuid::Uuid }

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct Users { pub users: Vec<UserRes> }


#[derive(Serialize, Deserialize, Type, Debug)]
pub struct User {
    pub user_id:        uuid::Uuid,
    pub user_name:      String,
    pub passhash:       String,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct UserRes {
    pub user_id:        uuid::Uuid,
    pub user_name:      String,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct Cells {
    pub cells:          Vec<CellExtracted>,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct CellReq {
    pub cell_id:        uuid::Uuid,
    pub user_id:        uuid::Uuid,
    pub device_id:      String,
    pub text:           String,
    pub is_open:        bool,
    pub fileprops:      Vec<FileProp>,
    pub parent_ids:     Vec<uuid::Uuid>,
    pub child_ids:      Vec<uuid::Uuid>,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct CellExtracted {
    pub cell_id:        uuid::Uuid,
    pub user_id:        uuid::Uuid,
    pub device_id:      String,
    pub text:           String,
    pub is_open:        bool,
    pub fileprops:      Vec<FileProp>,
    pub parents:        Vec<CellExtracted>,
    pub children:       Vec<CellExtracted>,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct CellFilter {
    pub user_id:        uuid::Uuid,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct FileProp {
    pub path:           String,
    pub url:            String,
    pub completed:      bool,
}