use crate::AppState;
use axum::{routing::get, Router};
use lazy_static::lazy_static;

mod status;

lazy_static! {
    pub static ref API_ROUTER: Router<AppState> =
        Router::new().route("/status", get(status::status));
}
