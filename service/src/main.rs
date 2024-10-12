use std::panic::set_hook;

use axum::Router;
use lambda_http::{run, tracing, Error};
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

    set_hook(Box::new(panic_hook));
    #[cfg(feature = "local")]
    let _ = axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app(),
    )
    .await;
    #[cfg(feature = "lambda")]
    run(app()).await
}

fn app() -> Router {
    let mut app = Router::new().route_service("/favicon.ico", ServeFile::new("favicon.ico"));
    for route in routes::get_all() {
        app = app.merge(route);
    }
    app
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use tokio::net::TcpListener;

    use crate::app;

    pub async fn spawn_server() -> SocketAddr {
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app()).await.unwrap();
        });
        addr
    }
}
