use crate::AppState;
use axum::{routing::get, Router};
use lazy_static::lazy_static;

mod board_index;
mod landing;

lazy_static! {
    pub static ref TEMPLATES_ROUTER: Router<AppState> = Router::new()
        .route("/landing", get(landing::landing))
        .route("/board_index", get(board_index::board_index));
}
