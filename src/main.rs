use axum::{
    http::{header, HeaderValue},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("public").not_found_service(ServeFile::new("public/index.html"));

    let app = Router::new()
        .nest_service(
            "/", // Serve files at the root URL.
            serve_dir,
        )
        .layer(SetResponseHeaderLayer::appending(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'none'; script-src 'self'; connect-src 'self'; img-src 'self'; style-src 'self'; frame-ancestors 'self'; form-action 'self'"),
        ))
        .route("/config", get(|| async { "hello world" }));

    // Define the socket address for the server.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Serving at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Serve the app on the specified address.
    axum::serve(listener, app).await.unwrap();
}
