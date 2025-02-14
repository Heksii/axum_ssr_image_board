use axum::{
    body::Body,
    debug_handler,
    extract::Query,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Board {
    id: i32,
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBoardPayload {
    title: String,
}

pub async fn create_board(
    Extension(pg_pool): Extension<PgPool>,
    Json(payload): Json<CreateBoardPayload>,
) -> Response {
    let title = payload.title;

    let query = sqlx::query!(
        "INSERT INTO boards (title) VALUES ($1) RETURNING id;",
        title.clone() as String
    );

    let query_result = query.fetch_one(&pg_pool).await;

    match query_result {
        Ok(record) => Response::builder()
            .status(201)
            .body(Body::from(format!(
                "Board with name '{0}' and id {1} was created.",
                title, record.id
            )))
            .unwrap(),
        Err(err) => Response::builder()
            .status(400)
            .body(Body::from(format!(
                "Unable to create board: {0}",
                err.to_string()
            )))
            .unwrap(),
    }
}

#[derive(Deserialize)]
pub struct Pagination {
    page: usize,
    per_page: usize,
}

#[debug_handler]
pub async fn list_boards(
    Extension(pg_pool): Extension<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Response {
    if pagination.per_page > 150 {
        return Response::builder()
            .status(400)
            .body(Body::from("Max page size is 150"))
            .unwrap();
    }

    let boards: Vec<Board> = sqlx::query_as!(
        Board,
        "SELECT * from boards LIMIT $1 OFFSET $2",
        pagination.per_page as i64,
        (pagination.per_page * pagination.page) as i64
    )
    .fetch_all(&pg_pool)
    .await
    .unwrap();

    Json(boards).into_response()
}
