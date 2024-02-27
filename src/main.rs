use std::collections::HashMap;

use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use minijinja::{context, Environment};
use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use serde::{Deserialize, Serialize};

pub mod db;

use db::read;

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();
    let serve_dir_from_dist = ServeDir::new("dist");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .nest_service("/dist", serve_dir_from_dist);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> Html<String> {
    let mut env = Environment::new();

    env.add_template("root.tpl", include_str!("../tpl/root.tpl"))
        .unwrap();

    let entries = db::read("dnsmasq.leases").unwrap();

    let template = env.get_template("root.tpl").unwrap();
    Html(template.render(context! { entries => entries }).unwrap())
}
