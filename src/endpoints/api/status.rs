use axum::{body::Body, response::Response};

pub async fn status() -> Response {
    Response::builder()
        .status(200)
        .body(Body::new("Up and running!".to_string()))
        .unwrap()
}
