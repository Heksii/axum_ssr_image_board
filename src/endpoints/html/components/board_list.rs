use axum::{
    response::{Html, IntoResponse, Response},
    Extension,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tera::Context;

use crate::TEMPLATES;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Board {
    board_id: i32,
    board_name: String,
}

pub async fn board_list(Extension(pg_pool): Extension<PgPool>) -> Response {
    let boards: Vec<Board> = sqlx::query_as!(Board, "SELECT * from boards")
        .fetch_all(&pg_pool)
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("boards", &boards);

    let html = TEMPLATES
        .render("components/board_list.tera", &context)
        .unwrap();

    Html(html).into_response()
}
