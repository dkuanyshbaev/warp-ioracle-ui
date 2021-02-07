use crate::{errors, models};
use askama::Template;
use sqlx::sqlite::SqlitePool;
use tokio::io::{AsyncWriteExt, ErrorKind};
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
    if let Ok(mut stream) = UnixStream::connect(IORACLE_SEND).await {
        loop {
            // check readiness
            if let Err(_) = stream.writable().await {
                continue;
            };

            match stream.try_write(b"read") {
                Ok(_) => {
                    println!("--------------->>>>>>>>> succ write");
                    if let Err(e) = stream.shutdown().await {
                        println!("{:?}", e);
                    };

                    break;
                }
                // will fail with `WouldBlock` if the readiness event is a false positive
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    // return Err(e.into());
                    println!("{:?}", e);
                    break;
                }
            }
        }
    };

    // use std::fs;
    use std::path::Path;
    let socket = Path::new(IORACLE_RETURN);
    // Delete old socket if necessary
    if socket.exists() {
        if let Err(error) = std::fs::remove_file(IORACLE_RETURN) {
            println!("{}", error);
            std::process::exit(1);
        };
    }

    println!("1");
    if let Ok(listener) = UnixListener::bind(IORACLE_RETURN) {
        println!("2");
        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    println!("new client!");
                    loop {
                        // check readiness
                        if let Err(_) = stream.readable().await {
                            continue;
                        };

                        // Creating the buffer **after** the `await` prevents it from
                        // being stored in the async task.
                        let mut buf = [0; 13];

                        match stream.try_read(&mut buf) {
                            // Ok(0) => break,
                            Ok(0) => continue,
                            Ok(n) => {
                                println!(">>>>>>>>>>>>>>>>>>> read {} bytes {:?}", n, buf);
                                let s = match std::str::from_utf8(&buf) {
                                    Ok(v) => v,
                                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                                };

                                println!("result: {}", s);
                            }
                            // will fail with `WouldBlock` if the readiness event is a false positive
                            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                                continue;
                            }
                            Err(e) => {
                                // return Err(e.into());
                                println!("{:?}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("connection failed! {:?}", e);
                }
            }
        }

        // let mut incoming = listener.incoming();
        // while let Some(stream) = listener.next().await {
        //     match stream {
        //         Ok(stream) => {
        //             println!("new client!");
        //         }
        //         Err(e) => {
        //             println!("connection failed!");
        //         }
        //     }
        // }
    };

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
    println!("{:?}", question);

    // --------------------------------------------------------------

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
