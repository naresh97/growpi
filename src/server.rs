use axum::{response::Html, routing::get, Router};

pub async fn run_server() {
    let app: Router = setup_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2205").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn setup_router() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Html<&'static str> {
    Html("hi")
}
