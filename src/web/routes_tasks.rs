
use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::{json, Value};

use crate::ctx::Ctx;
use crate::errors::{Error, Result};
use crate::web::db::Db;
use crate::web::tasks::{
    CreateTaskPayload, Task, TaskListQueryParams, UpdateTaskPayload,
};

pub fn routes(db: Db) -> Router {
    Router::new()
        .route("/tasks", post(create_task).get(list_tasks))
        .route(
            "/tasks/{id}",
            get(get_task_by_id)
                .put(update_task)
                .delete(delete_task),
        )
        .with_state(db)
}

async fn create_task(
    ctx: Ctx,
    State(db): State<Db>,
    Json(payload): Json<CreateTaskPayload>,
) -> Result<Json<Value>> {
    let user_id = ctx.user.id;

    let task = sqlx::query_as!(
        Task,
        r#"
        INSERT INTO tasks (project_id, title, description, due_date)
        SELECT $1, $2, $3, $4
        WHERE EXISTS (
            SELECT 1 FROM projects
            WHERE id = $1 AND user_id = $5
        )
        RETURNING id, project_id, title, description, status AS "status: _", due_date, created_at, updated_at
        "#,
        payload.project_id,
        payload.title,
        payload.description,
        payload.due_date,
        user_id
    )
    .fetch_optional(&db)
    .await?
    .ok_or(Error::ProjectUnauthorized)?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "task": task
        }
    })))
}

async fn list_tasks(
    ctx: Ctx,
    State(db): State<Db>,
    Query(params): Query<TaskListQueryParams>,
) -> Result<Json<Value>> {
    let user_id = ctx.user.id;
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let status_filter = params.status.unwrap_or_else(|| "%".to_string());

    let tasks = sqlx::query_as!(
        Task,
        r#"
        SELECT t.id, t.project_id, t.title, t.description, t.status AS "status: _", t.due_date, t.created_at, t.updated_at
        FROM tasks t
        JOIN projects p ON t.project_id = p.id
        WHERE p.user_id = $1 AND t.status ILIKE $2
        ORDER BY t.created_at DESC
        LIMIT $3 OFFSET $4
        "#,
        user_id,
        status_filter,
        limit as i64,
        offset as i64
    )
    .fetch_all(&db)
    .await?;

    let total_tasks = sqlx::query_scalar!(
        "SELECT COUNT(t.id) FROM tasks t JOIN projects p ON t.project_id = p.id WHERE p.user_id = $1",
        user_id
    )
    .fetch_one(&db)
    .await?
    .unwrap_or(0);

    Ok(Json(json!({
        "status": "success",
        "data": {
            "tasks": tasks,
            "pagination": {
                "total": total_tasks,
                "page": page,
                "limit": limit,
                "total_pages": (total_tasks as f64 / limit as f64).ceil() as u32
            }
        }
    })))
}

async fn get_task_by_id(
    ctx: Ctx,
    State(db): State<Db>,
    Path(task_id): Path<i64>,
) -> Result<Json<Value>> {
    let user_id = ctx.user.id;

    let task = sqlx::query_as!(
        Task,
        r#"
        SELECT t.id, t.project_id, t.title, t.description, t.status AS "status: _", t.due_date, t.created_at, t.updated_at
        FROM tasks t
        JOIN projects p ON t.project_id = p.id
        WHERE t.id = $1 AND p.user_id = $2
        "#,
        task_id,
        user_id
    )
    .fetch_optional(&db)
    .await?
    .ok_or(Error::TaskNotFound)?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "task": task
        }
    })))
}

async fn update_task(
    ctx: Ctx,
    State(db): State<Db>,
    Path(task_id): Path<i64>,
    Json(payload): Json<UpdateTaskPayload>,
) -> Result<Json<Value>> {
    let user_id = ctx.user.id;

    let task = sqlx::query_as!(
        Task,
        r#"
        UPDATE tasks t
        SET
            title = COALESCE($1, t.title),
            description = COALESCE($2, t.description),
            status = COALESCE($3, t.status),
            due_date = COALESCE($4, t.due_date),
            updated_at = CURRENT_TIMESTAMP
        FROM projects p
        WHERE
            t.id = $5 AND
            t.project_id = p.id AND
            p.user_id = $6
        RETURNING t.id, t.project_id, t.title, t.description, t.status AS "status: _", t.due_date, t.created_at, t.updated_at
        "#,
        payload.title,
        payload.description,
        payload.status.map(|s| s.to_string()),
        payload.due_date,
        task_id,
        user_id
    )
    .fetch_optional(&db)
    .await?
    .ok_or(Error::TaskUnauthorized)?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "task": task
        }
    })))
}

async fn delete_task(
    ctx: Ctx,
    State(db): State<Db>,
    Path(task_id): Path<i64>,
) -> Result<Json<Value>> {
    let user_id = ctx.user.id;

    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM tasks t
        USING projects p
        WHERE
            t.id = $1 AND
            t.project_id = p.id AND
            p.user_id = $2
        "#,
        task_id,
        user_id
    )
    .execute(&db)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(Error::TaskUnauthorized);
    }

    Ok(Json(json!({
        "status": "success",
        "message": "Task deleted successfully"
    })))
}
