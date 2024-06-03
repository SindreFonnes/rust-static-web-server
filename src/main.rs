use axum::{http::header, routing::get, Router};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

use axum::response::Json;
use serde_json::{json, Value};

mod config_parser;
mod csp;
mod enviroment;

use config_parser::read_config_file;
use csp::get_default_csp;
use enviroment::Environment;

#[tokio::main]
async fn main() {
    let environment = Environment(std::env::var("ENV").unwrap_or("local".into()));

    let config = match read_config_file(environment.clone()) {
        Ok(config) => config,
        Err(error) => {
            println!("{error}");
            Value::Null
        }
    };

    let client_config = match config.get("CLIENT") {
        Some(client) => Json(client.clone()),
        None => Json(json!({})),
    };

    let server_config = match config.get("SERVER") {
        Some(server) => server.clone(),
        None => {
            println!("Server config not found.");
            Value::Null
        }
    };

    let csp_config = csp::get_csp_rules(&server_config).unwrap_or_else(|| get_default_csp());

    let address = match server_config.get("PORT") {
        Some(port) => format!("0.0.0.0:{port}"),
        None => "0.0.0.0:8080".to_owned(),
    };

    let serve_dir = ServeDir::new("public").not_found_service(ServeFile::new("public/index.html"));

    let app = Router::new()
        .nest_service("/", serve_dir)
        .layer(SetResponseHeaderLayer::appending(
            header::CONTENT_SECURITY_POLICY,
            csp_config,
        ))
        .route("/config", get(|| async { client_config }))
        .route("/ping", get(|| async { "pong" }))
        .route("/healthz", get(|| async { "ok" }));

    let app = app.with_state(environment);

    println!("Serving at http://{address}");

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    // Serve the app on the specified address.
    axum::serve(listener, app).await.unwrap();
}
