use axum::response::{Html, IntoResponse, Response};
use tera::Context;

use crate::TEMPLATES;

pub async fn board_index() -> Response {
    let context = Context::new();
    let html = TEMPLATES
        .render("pages/board_index.tera", &context)
        .unwrap();

    Html(html).into_response()
}
