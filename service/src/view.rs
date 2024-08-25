use std::env;

use axum::response::IntoResponse;
use maud::{html, Markup, DOCTYPE};

pub fn page(clicked: bool, stage: String) -> Markup {
    html! {
        (DOCTYPE)
        (header())
        (body(clicked, stage))
    }
}

fn header() -> Markup {
    let htmx_url = env::var("HTMX_URL").expect("HTMX URL not defined");
    let htmx_integrity = env::var("HTMX_INTEGRITY").expect("HTMX integrity SHA not defined");
    html! {
        head {
            meta charset="utf-8";
            script src=(htmx_url) integrity=(htmx_integrity) crossorigin="anonymous" {}
            title { "Serverless" }
        }
    }
}

fn body(clicked: bool, stage: String) -> Markup {
    match clicked {
        true => html! {
            body {
                h1 { "RESTful HTML" }
                (component())
            }
        },
        false => html! {
            body {
                h1 { "RESTful HTML" }
                (button(stage))
            }
        },
    }
}

fn button(stage: String) -> Markup {
    let url = format!("{}/api/clicked", stage.to_string());
    html! {
        button id="component" hx-post=(url) hx-swap="outerHTML" hx-push-url="/clicked" { "Click me" }
    }
}

pub fn component() -> Markup {
    html! {
        h2 { "CLICKED" }
    }
}
