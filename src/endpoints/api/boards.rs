use axum::{
    body::Body,
    debug_handler,
    extract::{Query, RawPathParams},
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Board {
    board_id: i32,
    board_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBoard {
    board_name: String,
}

pub async fn create_board(
    Extension(pg_pool): Extension<PgPool>,
    Json(payload): Json<CreateBoard>,
) -> Response {
    let board_name = payload.board_name;

    match sqlx::query!(
        "INSERT INTO boards (board_name) VALUES ($1) RETURNING board_id;",
        board_name.clone() as String
    )
    .fetch_one(&pg_pool)
    .await
    {
        Ok(record) => {
            let new_id = record.board_id;

            return Response::builder()
                .status(201)
                .body(Body::from(format!(
                    "Board with name '{board_name}' and id {new_id} was created."
                )))
                .unwrap();
        }
        Err(err) => Response::builder()
            .status(400)
            .body(Body::from(format!(
                "Unable to create board: {0}",
                err.to_string()
            )))
            .unwrap(),
    }
}

#[debug_handler]
pub async fn list_boards(Extension(pg_pool): Extension<PgPool>) -> Response {
    let boards: Vec<Board> = sqlx::query_as!(Board, "SELECT * from boards")
        .fetch_all(&pg_pool)
        .await
        .unwrap();

    Json(boards).into_response()
}
