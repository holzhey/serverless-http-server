use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::MethodRouter;
use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use lambda_http::request::RequestContext::ApiGatewayV1;
use lambda_http::tracing;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::view::{component, page};

#[derive(Deserialize, Serialize)]
struct Params {
    first: Option<String>,
    second: Option<String>,
}

async fn root(req: axum::extract::Request) -> Markup {
    tracing::info!("Root");
    let context = req
        .extensions()
        .get::<lambda_http::request::RequestContext>();
    if let Some(ApiGatewayV1(ctx)) = context {
        page(false, format!("{:?}", ctx.stage.clone().unwrap()))
    } else {
        html!("Not API GW")
    }
}

async fn root_clicked(req: axum::extract::Request) -> Markup {
    tracing::info!("Root Clicked");
    let context = req
        .extensions()
        .get::<lambda_http::request::RequestContext>();
    if let Some(ApiGatewayV1(ctx)) = context {
        page(true, format!("{:?}", ctx.stage.clone().unwrap()))
    } else {
        html!("Not API GW")
    }
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

pub fn get_all() -> Vec<Router> {
    vec![
        route("/", get(root)),
        route("/clicked", get(root_clicked)),
        route("/api/clicked", get(clicked)),
        route("/foo", get(get_foo).post(post_foo)),
        route("/foo/:name", post(post_foo_name)),
        route("/parameters", get(get_parameters)),
        route("/health/", get(health_check)),
    ]
}

fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

#[cfg(test)]
mod tests {
    use crate::tests::spawn_server;

    #[tokio::test]
    async fn root_path() {
        let addr = spawn_server().await;
        let resp = reqwest::get(format!("http://{addr}")).await.unwrap();

        assert_eq!(resp.status(), 200);
        assert_eq!(resp.text().await.unwrap(), "Not API GW");
    }
}
