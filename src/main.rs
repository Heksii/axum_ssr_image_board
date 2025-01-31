mod endpoints;
use endpoints::{api::API_ROUTER, templates::TEMPLATES_ROUTER};

use axum::{response::Redirect, routing::get, Extension, Router};
use lazy_static::lazy_static;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{
    net::{IpAddr, SocketAddr},
    sync::{Arc, Mutex},
};
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
struct ActiveClient {
    ip: IpAddr,
    current_board_id: Option<u64>,
}

#[derive(Clone)]
pub struct AppState {
    active_clients: Arc<Mutex<Vec<ActiveClient>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            active_clients: Arc::new(Mutex::new(vec![])),
        }
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

    let base_router: Router<AppState> = Router::new()
        .nest_service("/static", ServeDir::new("src/static/"))
        .route("/", get(|| async { Redirect::permanent("/landing") }));

    let templates_router = TEMPLATES_ROUTER.clone();
    let api_router = API_ROUTER.clone();

    let app = Router::<AppState>::new()
        .merge(base_router)
        .merge(api_router)
        .merge(templates_router)
        .layer(Extension(db))
        .with_state(AppState::default())
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
