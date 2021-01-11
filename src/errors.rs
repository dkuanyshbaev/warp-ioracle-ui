use serde_derive::Serialize;
use std::convert::Infallible;
use warp::{http::StatusCode, Rejection, Reply};

// MetaWeather errors
// TODO: need to improve
#[derive(Debug)]
pub struct MetaWeatherError;
impl warp::reject::Reject for MetaWeatherError {}

// Openweather errors
// TODO: need to improve
#[derive(Debug)]
pub struct OpenWeatherError;
impl warp::reject::Reject for OpenWeatherError {}

// #[derive(Debug)]
// pub struct TemplateError;
// impl warp::reject::Reject for TemplateError {}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

// This is our rejections for warp
pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<MetaWeatherError>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Metaweather error";
    } else if let Some(_) = err.find::<OpenWeatherError>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Openweather error";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
