use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use worker::*;

// モジュール定義
mod handlers;
mod request;

use handlers::{create_todo, delete_todo, get_todos, root, update_todo};

fn router() -> Router {
    // CORS設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    Router::new()
        .route("/", get(root))
        .route("/api/todos", get(get_todos))
        .route("/api/todos", post(create_todo))
        .route("/api/todos/{id}", put(update_todo))
        .route("/api/todos/{id}", delete(delete_todo))
        .layer(cors)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}
