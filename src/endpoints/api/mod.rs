use crate::AppState;
use axum::{routing::get, Router};

mod boards;
mod status;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/api/status", get(status::status))
        .route(
            "/api/boards",
            get(boards::list_boards).post(boards::create_board),
        )
}
