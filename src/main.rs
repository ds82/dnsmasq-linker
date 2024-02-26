use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub mod db;

use db::read;

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> String {
    let line = db::read("dnsmasq.leases")
        .map(|v| {
            v.into_iter()
                .map(|entry| String::from(""))
                .collect::<Vec<String>>()
        })
        .unwrap();

    line.join("\n")
}
