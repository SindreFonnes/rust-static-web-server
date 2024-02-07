use axum::{
    extract::State,
    http::{header, HeaderValue},
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

use axum::response::{Json, ErrorResponse};
use serde_json::{Value, json};

async fn config(state: State<String>) -> impl IntoResponse {
    let env = state.as_str();
    let config_file = std::fs::read_to_string(format!("./config/{env}.json"));

    let result = match config_file {
        Ok(config_file_as_string) => config_file_as_string,
        Err(_) => "Internal Server Error".to_owned(),
    };

    let config = serde_json::from_str(&result);

    let mut config: Value = match config {
        Ok(config) => config,
        Err(_) => Value::Null,
    };

    if config.is_null() {
        return Json(json!({
            "error": "Failed to parse config file"
        }))
    }

	config["env"] = json!(env);

	Json(config)
}

#[derive(Debug, Clone)]
struct Environment(String);

#[tokio::main]
async fn main() {
    let address = std::env::var("ADDRESS").unwrap_or("localhost:3000".into());
    let environment = Environment(std::env::var("ENV").unwrap_or("local".into()));

    let serve_dir = ServeDir::new("public").not_found_service(ServeFile::new("public/index.html"));

	let app = Router::new()
		.nest_service(
			"/",
			serve_dir,
		)
		.layer(SetResponseHeaderLayer::appending(
			header::CONTENT_SECURITY_POLICY,
			HeaderValue::from_static("default-src 'none'; script-src 'self'; connect-src 'self'; img-src 'self'; style-src 'self'; frame-ancestors 'self'; form-action 'self'"),
		))
		.route("/config", get(config));

	let app = app.with_state(environment.0);

	println!("Serving at http://{address}");

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    // Serve the app on the specified address.
    axum::serve(listener, app).await.unwrap();
}
