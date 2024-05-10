use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::options::AggregateOptions;
use futures_util::stream::StreamExt;
use futures_util::AsyncWriteExt;
use futures_util::AsyncReadExt;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub name:           String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub name:           String,
    pub passhash:       String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub user_id:        ObjectId,
    pub device_id:      ObjectId,
    pub dir_ids:        Vec<ObjectId>,
    pub fileprop_ids:   Vec<ObjectId>,
    pub ancestor_ids:   Vec<ObjectId>,
    pub text:           String,
    pub is_open:        bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub user_id:        ObjectId,
    pub name:           String,
    pub dir_ids:        Vec<ObjectId>,
    pub fileprop_ids:   Vec<ObjectId>,   
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileProp {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:             Option<ObjectId>,
    pub user_id:        ObjectId,
    pub name:           String,
    pub blob_id:        ObjectId,
}


#[tokio::main]
async fn main() {
    
    // Connect DB
    let db_client = mongodb::Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db = db_client.database("flowfs");

    // GridFSバケットが無ければ作り、あれば既存のGridFSバケットへの参照を作成.
    let bucket = db.gridfs_bucket(None);

    // Upload
    let file_bytes = fs::read("./example.txt").await.unwrap();
    let mut upload_stream = bucket.open_upload_stream("example3.txt", None);
    upload_stream.write_all(&file_bytes[..]).await.unwrap();
    let us_id = upload_stream.id().clone();
    println!("Document uploaded with ID: {}", &us_id);
    upload_stream.close().await.unwrap();

    // GetInfo
    let mut cursor = bucket.find(doc! {"_id": us_id.clone()}, None).await.unwrap();
    let Some(result) = cursor.next().await else { return };
    let fcd = result.unwrap();
    let filename = &fcd.filename.unwrap();
    println!("File length: {}\n", &fcd.length);
    println!("File name: {}\n", &filename);

    // Upload 2
    let file_bytes = fs::read("./abcdefghi.py").await.unwrap();
    let mut upload_stream = bucket.open_upload_stream("abcdefghi.py", None);
    upload_stream.write_all(&file_bytes[..]).await.unwrap();
    let us_id_2 = upload_stream.id().clone();
    println!("Document uploaded with ID: {}", &us_id_2);
    upload_stream.close().await.unwrap();

    // GetInfo 2
    let mut cursor2 = bucket.find(doc! {"_id": us_id_2.clone()}, None).await.unwrap();
    let Some(result2) = cursor2.next().await else { return };
    let fcd2 = result2.unwrap();
    let filename2 = &fcd2.filename.unwrap();
    println!("File length: {}\n", &fcd2.length);
    println!("File name: {}\n", &filename2);

    // Rename
    // let new_name = "new_file_name.txt";
    // bucket.rename(us_id.clone(), &new_name).await.unwrap();

    // Download
    /*
    let mut buf: Vec<u8> = Vec::new();
    let mut download_stream = bucket.open_download_stream(us_id.clone()).await.unwrap();
    let result1 = download_stream.read_to_end(&mut buf).await.unwrap();
    println!("DOWN: {:?}", result1);
    println!("buff: {:?}", String::from_utf8_lossy(&buf));
    */

    // For test
    let user_ = User {
        id: None,
        name: "Suzuki".to_string(),
        passhash: "baaaaab".to_string(),
    };

    let dev_ = Device {
        id: None,
        name: "Note-PC1".to_string(),
    };
    
    // Insert test 1
    let u_id = db.collection("users").insert_one(user_, None).await.unwrap().inserted_id;
    let d_id = db.collection("devices").insert_one(dev_, None).await.unwrap().inserted_id;

    println!("{}, {}", u_id, d_id);

    // File create
    let fileprop = FileProp {
        id:             None,
        user_id:        u_id.as_object_id().unwrap(),
        name:           filename.to_string(),
        blob_id:        us_id.clone().as_object_id().unwrap(),
    };
    let f_id = db.collection("fileprops").insert_one(fileprop, None).await.unwrap().inserted_id;
 
    // File create 2
    let fileprop2 = FileProp {
        id:             None,
        user_id:        u_id.as_object_id().unwrap(),
        name:           filename2.to_string(),
        blob_id:        us_id_2.clone().as_object_id().unwrap(),
    };
    let f_id_2 = db.collection("fileprops").insert_one(fileprop2, None).await.unwrap().inserted_id;

    // For test 2
    let req_json = Cell {
        id: None,
        user_id:        u_id.as_object_id().unwrap(),
        device_id:      d_id.as_object_id().unwrap(),
        dir_ids:        vec![],
        fileprop_ids:   vec![f_id.as_object_id().unwrap(), f_id_2.as_object_id().unwrap()],
        ancestor_ids:   vec![],
        text:           "This is a test.".to_string(),
        is_open:        false,
    };

    // Get test 2
    let l_id = db.collection("cells").insert_one(req_json, None).await.unwrap().inserted_id;

    // Get test
    let cells = db.collection::<Cell>("cells");

    let pipeline: Vec<mongodb::bson::Document> = vec![
        doc! {
            "$match": {
                "_id": l_id,
            }
        },
        doc! {
            "$graphLookup": {
                "from": "devices",
                "startWith": "$device_id",
                "connectFromField": "device_id",
                "connectToField": "_id",
                "as": "device",
            }
        },
        doc! {
            "$graphLookup": {
                "from": "users",
                "startWith": "$user_id",
                "connectFromField": "user_id",
                "connectToField": "_id",
                "as": "user"
            }
        },
        doc! {
            "$graphLookup": {
                "from": "fileprops",
                "startWith": "$fileprop_ids",
                "connectFromField": "fileprop_ids",
                "connectToField": "_id",
                "as": "fileprops"
            }

        },
        doc! {
            "$project": {
                "user.passhash": 0,
            }
        },
    ];

    // 集計オプションを設定する
    let options = AggregateOptions::builder().build();

    // cellsコレクションで集計パイプラインを実行する
    let mut cursor = cells.aggregate(pipeline, options).await.unwrap();

    // 結果を処理する
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                // ドキュメントを処理する
                println!("{:?}", document);
            }
            Err(e) => {
                // エラーを処理する
                eprintln!("Error: {}", e);
            }
        }
    }

}
