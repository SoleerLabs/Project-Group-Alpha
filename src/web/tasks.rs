use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "task_status", rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Pending => "pending".to_string(),
            TaskStatus::InProgress => "in_progress".to_string(),
            TaskStatus::Completed => "completed".to_string(),
        }
    }
}

#[derive(Debug, Serialize, FromRow, Clone, ToSchema)]
pub struct Task {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTaskPayload {
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTaskPayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TaskListQueryParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<TaskStatus>,
}
