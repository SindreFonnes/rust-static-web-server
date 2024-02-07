use axum::{
    extract::State,
    http::{header, HeaderValue, StatusCode},
    routing::get,
    Router,
};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

use axum::response::Json;
use serde_json::{json, Value};

async fn config(
    State(Environment(env)): State<Environment>,
) -> Result<Json<Value>, (StatusCode, &'static str)> {
    let config_file = std::fs::read_to_string(format!("./config/{env}.json")).map_err(|error| {
        println!("{error}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to load config file.",
        )
    })?;

    let mut config: Value = serde_json::from_str(&config_file).map_err(|error| {
        println!("{error}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to parse config file.",
        )
    })?;

    if config.is_null() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Config should not be null.",
        ));
    }

    config["env"] = json!(env);

    Ok(Json(config))
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

    let app = app.with_state(environment);

    println!("Serving at http://{address}");

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    // Serve the app on the specified address.
    axum::serve(listener, app).await.unwrap();
}
