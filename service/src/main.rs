use std::panic::set_hook;

use axum::Router;
use lambda_http::{run, tracing, Error};
use routes::get_all;
use tower_http::services::ServeFile;
use tracing_panic::panic_hook;

pub mod routes;
pub mod view;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_current_span(false)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .init();

    let mut app = Router::new().route_service("/favicon.ico", ServeFile::new("favicon.ico"));
    for route in get_all() {
        app = app.merge(route);
    }
    set_hook(Box::new(panic_hook));
    run(app).await
}
