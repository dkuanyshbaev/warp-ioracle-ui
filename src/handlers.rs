use crate::errors;
use crate::models;
use askama::Template;
use sqlx::sqlite::SqlitePool;
use warp::{http::Uri, redirect, reject, reply, Rejection, Reply};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "answer.html")]
struct AnswerTemplate<'a> {
    name: &'a str,
}

pub async fn index() -> Result<impl Reply, Rejection> {
    let template = IndexTemplate;
    let response = template
        .render()
        .map_err(|_| reject::custom(errors::OpenWeatherError))?;

    Ok(reply::html(response))
}

pub async fn question(question: models::Question, db: SqlitePool) -> Result<impl Reply, Rejection> {
    println!("----------- {:?}", question);

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&db)
        .await
        .unwrap_or_else(|err| {
            println!("Test: {}", err);
            std::process::exit(1);
        });

    println!("{:?}", row);

    // Ok(reply::json(&"yo".to_string()))
    //
    use std::str::FromStr;
    let location = Uri::from_str(&format!("/answer/{}", "23")).unwrap();

    // Ok(redirect::(Uri::from_string("/answer")))

    Ok(redirect(location))
}

pub async fn answer(uuid: String, db: SqlitePool) -> Result<impl Reply, Rejection> {
    println!("----------- {:?}", uuid);

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&db)
        .await
        .unwrap_or_else(|err| {
            println!("Test: {}", err);
            std::process::exit(1);
        });

    println!("{:?}", row);

    let template = AnswerTemplate { name: "Denis" };
    let response = template
        .render()
        .map_err(|_| reject::custom(errors::OpenWeatherError))?;

    Ok(reply::html(response))
}
