use std::panic::set_hook;

use axum::middleware::from_fn;
use axum::Router;
use lambda_http::request::RequestContext::ApiGatewayV1;
use lambda_http::{run, tracing, Error};
use tower_http::services::ServeFile;
use tracing_panic::panic_hook;

pub mod routes;
pub mod view;

async fn mw_sample(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> impl axum::response::IntoResponse {
    let context = req
        .extensions()
        .get::<lambda_http::request::RequestContext>();
    tracing::info!("Context {:?}", context);
    if let Some(ApiGatewayV1(ctx)) = context {
        tracing::info!("RequestId = {:?}", ctx.request_id);
        tracing::info!("Stage= {:?}", ctx.stage);
    }
    next.run(req).await
}

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
    for route in routes::get_all() {
        app = app.merge(route);
    }
    app = app.route_layer(from_fn(mw_sample));
    set_hook(Box::new(panic_hook));
    run(app).await
}
