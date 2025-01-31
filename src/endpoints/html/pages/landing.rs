use axum::response::{Html, IntoResponse, Response};
use tera::Context;

use crate::TEMPLATES;

pub async fn landing() -> Response {
    let context = Context::new();
    let html = TEMPLATES.render("pages/landing.tera", &context).unwrap();

    Html(html).into_response()
}
