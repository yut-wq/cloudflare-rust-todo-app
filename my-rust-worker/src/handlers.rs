use axum::{extract::Path, http::StatusCode, response::Json};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::request::{CreateTodoRequest, Todo, UpdateTodoRequest};

// グローバルなTODOストレージ（スレッドセーフ）
lazy_static! {
    static ref TODOS: Mutex<HashMap<u64, Todo>> = Mutex::new(HashMap::new());
}

pub async fn root() -> &'static str {
    "TODO API Server"
}

// 全TODO取得
pub async fn get_todos() -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = TODOS
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let todo_list: Vec<Todo> = todos.values().cloned().collect();
    Ok(Json(todo_list))
}

// TODO作成
pub async fn create_todo(Json(request): Json<CreateTodoRequest>) -> Result<Json<Todo>, StatusCode> {
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
pub async fn update_todo(
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
pub async fn delete_todo(Path(id): Path<u64>) -> Result<StatusCode, StatusCode> {
    let mut todos = TODOS
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match todos.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(StatusCode::NOT_FOUND),
    }
}
