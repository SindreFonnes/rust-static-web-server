use axum::{
    http::{header, HeaderValue},
    routing::get,
    Router,
};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

use axum::response::Json;
use serde_json::{json, Value};

fn insert_config_overwrite(mut config: Value, key: String, value: String) -> Result<Value, String> {
    let sub_keys = key.split("__").collect::<Vec<&str>>();

    let mut current_sub_object: &mut Value = &mut config;

    for key in sub_keys {
        match current_sub_object.get_mut(key) {
            Some(value) => {
                current_sub_object = value;
            }
            None => {
                return Err("Key not found.".to_owned());
            }
        }
    }

    *current_sub_object = json!(value);

    Ok(config)
}

fn read_config_file(Environment(env): Environment) -> Result<Value, String> {
    let config_file = std::fs::read_to_string(format!("./config/{env}.json"))
        .map_err(|error| format!("Unable to load config file. {}", error))?;

    let mut config: Value = serde_json::from_str(&config_file)
        .map_err(|error| format!("Unable to parse config file. {}", error))?;

    if config.is_null() {
        return Err("Config should not be null.".to_owned());
    }

    if config.is_null() {
        return Err("Config should not be null.".to_owned());
    }

    std::env::vars().for_each(|(key, value)| {
        if key.starts_with("CONFIG__") {
            let key = key.replace("CONFIG__", "");

            match insert_config_overwrite(config.clone(), key, value) {
                Ok(next_config) => {
                    config = next_config;
                }
                Err(error) => {
                    println!("{error}");
                }
            }
        }
    });

    Ok(config)
}

#[derive(Debug, Clone)]
struct Environment(String);

#[tokio::main]
async fn main() {
    let address = std::env::var("ADDRESS").unwrap_or("localhost:3000".into());
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
		.route("/config", get(|| async { client_config }));

    let app = app.with_state(environment);

    println!("Serving at http://{address}");

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    // Serve the app on the specified address.
    axum::serve(listener, app).await.unwrap();
}
