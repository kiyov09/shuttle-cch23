use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::Deserialize;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[derive(Debug, Deserialize)]
struct RecalibrationPath {
    ids: String,
}

impl IntoIterator for RecalibrationPath {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.ids
            .split('/')
            .filter_map(|x| x.parse::<Self::Item>().ok())
            .collect::<Vec<_>>()
            .into_iter()
    }
}

// Day 1 - Task 1
async fn recalibrate(Path(recal): Path<RecalibrationPath>) -> impl IntoResponse {
    match recal
        .into_iter()
        .reduce(|a, b| a ^ b)
        .map(|x| x.pow(3).to_string())
    {
        Some(result) => (StatusCode::OK, result),
        None => (StatusCode::BAD_REQUEST, "Bad Request".to_string()),
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/*ids", get(recalibrate));

    Ok(router.into())
}
