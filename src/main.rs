use axum::{
    Router,
    routing::get,
    response::Html,
};

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

#[tokio::main]
async fn main() {
    // Build our application
    let app = Router::new()
        .route("/", get(index))
        .route("/healthcheck", get(|| async { "OK" }));

    // Run it on 0.0.0.0:8000 to be accessible from outside the container
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on http://0.0.0.0:8000");
    
    axum::serve(listener, app).await.unwrap();
}
