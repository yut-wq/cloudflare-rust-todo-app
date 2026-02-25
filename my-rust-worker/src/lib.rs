use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use worker::*;

// リクエスト定義をインポート
mod request;
use request::{CreateTodoRequest, Todo, UpdateTodoRequest};

// グローバルなTODOストレージ（スレッドセーフ）
lazy_static! {
    static ref TODOS: Mutex<HashMap<u64, Todo>> = Mutex::new(HashMap::new());
}

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

pub async fn root() -> &'static str {
    "TODO API Server"
}

// 全TODO取得
async fn get_todos() -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = TODOS
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let todo_list: Vec<Todo> = todos.values().cloned().collect();
    Ok(Json(todo_list))
}

// TODO作成
async fn create_todo(Json(request): Json<CreateTodoRequest>) -> Result<Json<Todo>, StatusCode> {
    let mut todos = TODOS
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 現在時刻をIDとして使用
    let id = js_sys::Date::now() as u64;

    let todo = Todo {
        id,
        text: request.text.trim().to_string(),
        completed: false,
    };

    if todo.text.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    todos.insert(id, todo.clone());
    Ok(Json(todo))
}

// TODO更新（完了状態の切り替え）
async fn update_todo(
    Path(id): Path<u64>,
    Json(request): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = TODOS
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match todos.get_mut(&id) {
        Some(todo) => {
            todo.completed = request.completed;
            Ok(Json(todo.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// TODO削除
async fn delete_todo(Path(id): Path<u64>) -> Result<StatusCode, StatusCode> {
    let mut todos = TODOS
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match todos.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(StatusCode::NOT_FOUND),
    }
}
