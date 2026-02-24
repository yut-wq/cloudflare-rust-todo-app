use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use worker::*;

// TODO構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}

// TODO作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub text: String,
}

// TODO更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub completed: bool,
}

// グローバル状態を単純化
static mut TODOS: Option<HashMap<u64, Todo>> = None;
static mut INITIALIZED: bool = false;

fn get_todos_store() -> &'static mut HashMap<u64, Todo> {
    unsafe {
        if !INITIALIZED {
            TODOS = Some(HashMap::new());
            INITIALIZED = true;
        }
        TODOS.as_mut().unwrap()
    }
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
    let todos = get_todos_store();
    let todo_list: Vec<Todo> = todos.values().cloned().collect();
    Ok(Json(todo_list))
}

// TODO作成
async fn create_todo(Json(request): Json<CreateTodoRequest>) -> Result<Json<Todo>, StatusCode> {
    let todos = get_todos_store();

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
    let todos = get_todos_store();

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
    let todos = get_todos_store();

    match todos.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(StatusCode::NOT_FOUND),
    }
}
