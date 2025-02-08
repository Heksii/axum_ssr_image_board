mod endpoints;
use endpoints::{api::api_router, html::html_router};

use axum::{response::Redirect, routing::get, Extension, Router};
use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tera::Tera;
use tower_http::services::ServeDir;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("src/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

#[derive(Clone)]
pub struct AppState {}

impl Default for AppState {
    fn default() -> Self {
        Self {}
    }
}

#[tokio::main]
async fn main() {
    let db_url = dotenvy::var("DATABASE_URL").expect("'DATABASE_URL' was not set in .env");

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .merge(api_router())
        .merge(html_router())
        .route("/", get(|| async { Redirect::permanent("/pages/landing") }))
        .nest_service("/static", ServeDir::new("src/static/"))
        .layer(Extension(db))
        .with_state(AppState::default())
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Listender failed to build");

    axum::serve(listener, app).await.expect("Failed to serve");
}
