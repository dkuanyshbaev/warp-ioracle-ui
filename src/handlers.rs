use crate::errors;
use crate::models;
use askama::Template;
use sqlx::sqlite::SqlitePool;
// use tokio::io::AsyncWriteExt;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
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
    // TODO: send message to core & wait for response

    println!("----------- {:?}", question);

    // --------------------------------------------------------------

    println!("send to gate");

    if let Ok(mut stream) = UnixStream::connect("/tmp/ioracle.sock").await {
        loop {
            // Wait for the socket to be writable
            if let Err(e) = stream.writable().await {
                println!("{:?}", e);
                break;
            };

            // Try to write data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match stream.try_write(b"hello world") {
                Ok(n) => {
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    // return Err(e.into());
                    println!("{:?}", e);
                    break;
                }
            }
        }
        // if let Err(e) = stream.write_all(b"read") {
        //     println!("{:?}", e);
        // };
    };

    // let mut listener = UnixListener::bind("/tmp/ioracle.out").unwrap();

    // if let Ok(mut stream) = UnixListener::bind("/tmp/ioracle.in") {
    //     if let Err(e) = stream.write_all(b"read") {
    //         println!("{:?}", e);
    //     };
    // };

    // --------------------------------------------------------------

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&db)
        .await
        .unwrap_or_else(|err| {
            println!("Test: {}", err);
            std::process::exit(1);
        });

    println!("{:?}", row);

    use std::str::FromStr;
    let location = Uri::from_str(&format!("/answer/{}", "23")).unwrap();

    Ok(redirect(location))
}

pub async fn answer(uuid: String, db: SqlitePool) -> Result<impl Reply, Rejection> {
    // TODO: get answer by uuid & show it

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
