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

    set_hook(Box::new(panic_hook));
    run(app()).await
}

fn app() -> Router {
    let mut app = Router::new().route_service("/favicon.ico", ServeFile::new("favicon.ico"));
    for route in routes::get_all() {
        app = app.merge(route);
    }
    app.route_layer(from_fn(mw_sample))
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use tokio::net::TcpListener;

    use crate::app;

    #[tokio::test]
    async fn test_root_path() -> httpc_test::Result<()> {
        let addr = spawn_server().await;
        let cli = httpc_test::new_client(format!("http://{addr}"))?;

        let resp = cli.do_get("/").await?;

        assert_eq!(resp.status(), 200);
        assert_eq!(resp.text_body().unwrap(), "Not API GW");
        Ok(())
    }

    async fn spawn_server() -> SocketAddr {
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app()).await.unwrap();
        });
        addr
    }
}
