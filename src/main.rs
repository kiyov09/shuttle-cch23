use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

// Day 1 - Task 1
async fn recalibrate(Path((num1, num2)): Path<(i32, i32)>) -> impl IntoResponse {
    let result = (num1 ^ num2).pow(3);
    (StatusCode::OK, result.to_string())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/:num1/:num2", get(recalibrate));

    Ok(router.into())
}
