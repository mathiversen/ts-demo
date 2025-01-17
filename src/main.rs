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
    let app = Router::new()
        .route("/", get(index))
        .route("/healthcheck", get(|| async { "OK" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on http://0.0.0.0:8000");
    
    axum::serve(listener, app).await.unwrap();
}
