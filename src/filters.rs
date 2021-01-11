use crate::handlers;
use crate::models;
use sqlx::sqlite::SqlitePool;
use warp::{Filter, Rejection, Reply};

pub fn static_files() -> impl Filter<Extract = (warp::fs::File,), Error = Rejection> + Clone {
    warp::path("static").and(warp::fs::dir("static"))
}

fn with_db(
    db: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

// fn json_body() -> impl Filter<Extract = (models::Question,), Error = Rejection> + Clone {
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }

fn form_body() -> impl Filter<Extract = (models::Question,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::form())
}

pub fn index() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end().and(warp::get()).and_then(handlers::index)
}

pub fn question(db: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("question")
        .and(warp::post())
        // .and(json_body())
        .and(form_body())
        .and(with_db(db))
        .and_then(handlers::question)
}

pub fn answer(db: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("answer")
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db))
        .and_then(handlers::answer)
}

pub fn ioracle(db: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    static_files()
        .or(index())
        .or(question(db.clone()))
        .or(answer(db.clone()))
}
