use crate::{errors, models};
use askama::Template;
use sqlx::sqlite::SqlitePool;
use std::{str, str::FromStr};
use tokio::io::ErrorKind;
use tokio::net::{UnixListener, UnixStream};
use warp::{http::Uri, redirect, reject, reply, Rejection, Reply};

const IORACLE_SEND: &str = "/tmp/ioracle.send";
const IORACLE_RETURN: &str = "/tmp/ioracle.return";

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
    if let Ok(stream) = UnixStream::connect(IORACLE_SEND).await {
        loop {
            if stream.writable().await.is_err() {
                continue;
            }

            match stream.try_write(b"read") {
                Ok(_) => break,

                // will fail with `WouldBlock` if the readiness event is a false positive
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }

                Err(_) => {
                    // return Err(?)
                    println!("Can't write to SEND socket");
                    break;
                }
            }
        }
    }

    let mut location = Uri::from_str("/").unwrap();

    if let Ok(listener) = UnixListener::bind(IORACLE_RETURN) {
        'connection: loop {
            if let Ok((stream, _)) = listener.accept().await {
                loop {
                    if stream.readable().await.is_err() {
                        continue;
                    }

                    let mut result = [0; 12];

                    match stream.try_read(&mut result) {
                        Ok(0) => continue,

                        Ok(_) => {
                            if let Ok(result) = std::str::from_utf8(&result) {
                                if let Some(uuid) = models::save(db, question, result) {
                                    location = Uri::from_str(&format!("/answer/{}", uuid)).unwrap();
                                }
                            }
                            break 'connection;
                        }

                        // will fail with `WouldBlock` if the readiness event is a false positive
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                            continue;
                        }

                        Err(_) => {
                            println!("Can't read from RETURN socket");
                            break 'connection;
                        }
                    }
                }
            }
        }
    }

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
