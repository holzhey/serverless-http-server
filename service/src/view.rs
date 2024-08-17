use std::env;

use maud::{html, Markup, DOCTYPE};

pub fn page(clicked: bool) -> Markup {
    html! {
        (DOCTYPE)
        (header())
        (body(clicked))
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

fn body(clicked: bool) -> Markup {
    let content = match clicked {
        true => component,
        false => button,
    };
    html! {
        body {
            h1 { "RESTful HTML" }
            (content())
        }
    }
}

fn button() -> Markup {
    html! {
        button id="component" hx-post="/api/clicked" hx-swap="outerHTML" hx-push-url="/clicked" { "Click me" }
    }
}

pub fn component() -> Markup {
    html! {
        h2 { "CLICKED" }
    }
}
