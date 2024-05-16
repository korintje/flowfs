use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};
use sqlx::{FromRow};

pub mod request;
pub use request::*;


#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct IdRes { pub _id: ObjectId }


#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Users { pub users: Vec<User> }


#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id:            Option<i64>,
    pub name:           String,
    pub passhash:       String,
    pub cells:          Vec<Cell>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Cell {
    pub _id:            i64,
    pub fileprops:      Vec<FileProp>,
    pub ancestor_ids:   Vec<i64>,
    pub text:           String,
    pub device_id:      String,
    pub is_open:        bool,
}


#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CellInfo {
    pub _id:            i64,
    pub text:           String,
    pub device_id:      String,
    pub is_open:        bool,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Cells {
    pub cells:          Vec<Cell>,
}

impl From<Cell> for CellInfo {
    fn from(cell: Cell) -> Self {
        CellInfo {
            _id: cell._id,
            text: cell.text,
            device_id: cell.device_id,
            is_open: cell.is_open,
        }
    } 
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Dir {
    pub name:           String,
    pub dirs:           Vec<Dir>,
    pub fileprops:      Vec<FileProp>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct FileProp {
    pub name:       String,
    pub file_url:   String,
    pub completed:  bool,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct FileBlob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id:            Option<i64>,
    pub user_id:        ObjectId,
    pub file_name:      String,
    pub blob:           Vec<u8>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct UpdateCellReq {
    pub _id:            i64,
    pub rootdir:        Option<Dir>,
    pub ancestor_ids:   Option<Vec<ObjectId>>,
    pub text:           Option<String>,
    pub is_open:        Option<bool>,
}