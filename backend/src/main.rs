use std::env;
use env_logger;
use mongodb::Collection;

mod utils;
mod model;
mod handler;
use model::User;
use handler::{
    user::{list_users, create_user, show_user, update_user, delete_user},
    cell::{create_cell, show_cell, update_cell, delete_cell},
    // dir::{list_dirs, create_dir, show_dir, update_dir, delete_dir},
};

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {

    // Initialize logger
    env_logger::init();

    // Set web URL
    let listen_url = utils::get_url();
    let addr = env::args().nth(1).unwrap_or_else(|| listen_url);

    // MongoDBクライアントのセットアップ
    // Ref: https://www.mongodb.com/docs/drivers/rust/current/quick-start/connect-to-mongodb/
    let db_url = utils::get_db_path();
    let db_client = mongodb::Client::with_uri_str(db_url).await.unwrap();
    let db = db_client.database("flowfs");
    let col: Collection<User> = db.collection("users");

    // PostgreSQLクライアントのセットアップ
    let db_url = utils::get_db_path();
    let pool = sqlx::postgres::PgPoolOptions::new().max_connections(100).connect(&db_url).await.unwrap();
    init_db(&pool).await;
    // sqlx::migrate!().run(&pool).await.unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        // .route("/users", get(list_users).post(create_user))
        // .route("/users/:user_id", get(show_user).put(update_user).delete(delete_user))
        .route("/users/:user_id/cells/", post(create_cell))
        // .route("/users/:user_id/cells/:cell_id", get(show_cell).put(update_cell).delete(delete_cell))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}


async fn init_db(db: &sqlx::pool::Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {

    let _r = sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            user_id           INTEGER NOT NULL PRIMARY KEY
            , name            TEXT
            , passhash        TEXT
            , created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(db)
    .await?;

    println!("{:?}", _r);

    let _r = sqlx::query(
        "CREATE TABLE IF NOT EXISTS cells (
            cell_id         INTEGER NOT NULL PRIMARY KEY
            , device_id     TEXT
            , text          TEXT
            , is_open       BOOLEAN
            , created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(db)
    .await?;

    let _r = sqlx::query(
        "CREATE TABLE IF NOT EXISTS fileprops (
            fileprop_id     INTEGER NOT NULL PRIMARY KEY
            , user_id       INTEGER NOT NULL REFERENCES users(user_id),
            , cell_id       INTEGER NOT NULL REFERENCES cells(cell_id),
            , name          TEXT
            , path          TEXT
            , created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(db)
    .await?;

    let _r = sqlx::query(
        "CREATE TABLE IF NOT EXISTS ancestor_ids (
            descendant_id     INTEGER NOT NULL REFERENCES cells(cell_id),
            , ancestor_id     INTEGER NOT NULL REFERENCES cells(cell_id),
        )"
    )
    .execute(db)
    .await?;

    Ok(())
}



/*

        match cmd {

            4 => {
                let req: FileProp = serde_json::from_slice(body).unwrap();
                let file_props = db.collection("file_props");
                let parent_id_wrap = req.parent_id;
                let file_prop_id = file_props.insert_one(req, None).await.unwrap().inserted_id;
                let file_prop_id = file_prop_id.as_object_id().unwrap();
                println!("FileProp created with ID: {}", &file_prop_id);
                if let Some(parent_id) = parent_id_wrap {
                    file_props.update_one(
                        doc! { "_id": parent_id },
                        doc! { "$push": {"files": file_prop_id} },
                        None,
                    ).await.unwrap();
                }
                response = serde_json::to_string(&CreateFilePropRes{file_prop_id}).unwrap();
            }
            5 => {
                let req: FileBlob = serde_json::from_slice(body).unwrap();
                let bucket = db.gridfs_bucket(None);
                let blob = req.blob;
                let mut upload_stream = bucket.open_upload_stream(req.file_name, None);
                upload_stream.write_all(&blob[..]).await.unwrap();
                let uploaded_id = upload_stream.id();
                let uploaded_id = uploaded_id.as_object_id().unwrap();
                upload_stream.close().await.unwrap();
                println!("File uploaded with ID: {}", &uploaded_id);
                response = serde_json::to_string(&UploadFileRes{uploaded_id}).unwrap();
            }

*/
