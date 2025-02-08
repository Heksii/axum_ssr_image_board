use crate::AppState;
use axum::{routing::get, Router};

mod components;
mod pages;

pub fn html_router() -> Router<AppState> {
    Router::new()
        .route("/pages/landing", get(pages::landing::landing))
        .route("/pages/board_index", get(pages::board_index::board_index))
        .route(
            "/components/board_list",
            get(components::board_list::board_list),
        )
}
