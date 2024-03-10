use axum::routing::get;

#[tokio::main]
async fn main() {
    // build the application
    let app = axum::Router::new()
        .fallback(fallback)
        .route("/hello/:name", get(hello))
        .route("/goodbye/:name", get(goodbye));

    // Run the application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

async fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

async fn goodbye(name: String) -> String {
    format!("Goodbye, {}!", name)
}
