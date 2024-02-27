use std::collections::HashMap;

use axum::{
    extract::State,
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
use std::env;

pub mod db;
pub mod utils;

use db::read;
use utils::{get_env, get_env_bool};

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub lease_file: String,
}

// type SharedState = State<Arc<Mutex<AppState>>>;

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();
    let serve_dir_from_dist = ServeDir::new("dist");

    let state = AppState {
        lease_file: get_env("LEASE_FILE").unwrap(),
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .nest_service("/dist", serve_dir_from_dist)
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(state: State<AppState>) -> Html<String> {
    let mut env = Environment::new();

    env.add_template("root.tpl", include_str!("../tpl/root.tpl"))
        .unwrap();

    let entries = db::read(&state.lease_file).unwrap();

    let template = env.get_template("root.tpl").unwrap();
    Html(template.render(context! { entries => entries }).unwrap())
}
