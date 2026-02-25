use serde::{Deserialize, Serialize};

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
