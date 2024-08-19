use axum::extract::Query;
use axum::http::StatusCode;
use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use lambda_http::{run, tracing, Error};
use maud::Markup;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::services::ServeFile;
use tracing_panic::panic_hook;
use view::{component, page};

pub mod view;

#[derive(Deserialize, Serialize)]
struct Params {
    first: Option<String>,
    second: Option<String>,
}

async fn root() -> Markup {
    tracing::info!("Root");
    page(false)
}

async fn root_clicked() -> Markup {
    tracing::info!("Root Clicked");
    page(true)
}

async fn clicked() -> Markup {
    component()
}

async fn get_foo() -> Json<Value> {
    Json(json!({ "msg": "I am GET /foo" }))
}

async fn post_foo() -> Json<Value> {
    Json(json!({ "msg": "I am POST /foo" }))
}

async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("I am POST /foo/:name, name={name}") }))
}

async fn get_parameters(Query(params): Query<Params>) -> Json<Value> {
    Json(json!({ "request parameters": params }))
}

/// Example on how to return status codes and data from an Axum function
#[tracing::instrument]
async fn health_check() -> (StatusCode, String) {
    let health = true;
    match health {
        true => (StatusCode::OK, "Healthy!".to_string()),
        false => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Not healthy!".to_string(),
        ),
    }
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

    let app = Router::new()
        .route("/", get(root))
        .route("/clicked", get(root_clicked))
        .route("/api/clicked", get(clicked))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/:name", post(post_foo_name))
        .route("/parameters", get(get_parameters))
        .route("/health/", get(health_check))
        .route_service("/favicon.ico", ServeFile::new("favicon.ico"));

    std::panic::set_hook(Box::new(panic_hook));
    run(app).await
}
