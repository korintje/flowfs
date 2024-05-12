use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};

pub mod request;
pub use request::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct IdRes { pub _id: ObjectId }


#[derive(Debug, Serialize, Deserialize)]
pub struct Root { pub users: Vec<User> }


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id:            Option<ObjectId>,
    pub name:           String,
    pub passhash:       String,
    pub cells:          Vec<Cell>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id:            Option<ObjectId>,
    pub rootdir:        Dir,
    pub ancestor_ids:   Vec<ObjectId>,
    pub text:           String,
    pub device_id:      String,
    pub is_open:        bool,
}

impl Into<mongodb::bson::Bson> for Cell {
    fn into(self) -> mongodb::bson::Bson {
        mongodb::bson::Bson::Document(doc! {
            "_id":            self._id,
            "rootdir":        self.rootdir,
            "ancestor_ids":   self.ancestor_ids,
            "text":           self.text,
            "device_id":      self.device_id,
            "is_open":        self.is_open,
        })
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Dir {
    pub name:           String,
    pub dirs:           Vec<Dir>,
    pub fileprops:      Vec<FileProp>,
}

impl Into<mongodb::bson::Bson> for Dir {
    fn into(self) -> mongodb::bson::Bson {
        mongodb::bson::Bson::Document(doc! {
            "name": self.name,
            "dirs": self.dirs,
            "fileprops": self.fileprops,
        })
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FileProp {
    pub name:           String,
    pub blob_id:        ObjectId,
    pub completed:      bool,
}

impl Into<mongodb::bson::Bson> for FileProp {
    fn into(self) -> mongodb::bson::Bson {
        mongodb::bson::Bson::Document(doc! {
            "name": self.name,
            "blob_id": self.blob_id,
            "completed": self.completed,
        })
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FileBlob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id:            Option<ObjectId>,
    pub user_id:        ObjectId,
    pub file_name:      String,
    pub blob:           Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCellReq {
    pub _id:            ObjectId,
    pub rootdir:        Option<Dir>,
    pub ancestor_ids:   Option<Vec<ObjectId>>,
    pub text:           Option<String>,
    pub is_open:        Option<bool>,
}