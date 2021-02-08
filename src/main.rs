mod errors;
mod filters;
mod handlers;
mod models;

use sqlx::sqlite::SqlitePool;
use std::path::Path;
use std::{env, fs, process};
use warp::Filter;

const IORACLE_RETURN: &str = "/tmp/ioracle.return";
const DB: &str = "./db/ioracle.db";

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let db = SqlitePool::connect(DB).await.unwrap_or_else(|_| {
        println!("Can't connect to DB!");
        process::exit(1);
    });

    let socket = Path::new(IORACLE_RETURN);
    if socket.exists() {
        if fs::remove_file(IORACLE_RETURN).is_err() {
            println!("Can't remove socket!");
            process::exit(1);
        }
    }

    let routes = filters::ioracle(db)
        .with(warp::log("ioracle"))
        .recover(errors::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 4444)).await;
}
