use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
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
            get(get_task_by_id).put(update_task).delete(delete_task),
        )
        .with_state(db)
}

async fn create_task(
    ctx: Ctx,
    State(db): State<Db>,
    Json(payload): Json<CreateTaskPayload>,
) -> Result<Json<Value>> {
    let project_user_id: i64 = sqlx::query_scalar("SELECT user_id FROM projects WHERE id = $1")
        .bind(payload.project_id)
        .fetch_optional(&db)
        .await?
        .ok_or(Error::ProjectNotFound)?;

    if project_user_id != ctx.user.id {
        return Err(Error::ProjectUnauthorized);
    }

    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (project_id, title, description, due_date) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(payload.project_id)
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.due_date)
    .fetch_one(&db)
    .await?;

    Ok(Json(json!({ "status": "success", "data": { "task": task } })))
}

async fn list_tasks(
    ctx: Ctx,
    State(db): State<Db>,
    Query(params): Query<TaskListQueryParams>,
) -> Result<Json<Value>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    // Base query
    let mut query_builder = sqlx::QueryBuilder::new("SELECT t.* FROM tasks t JOIN projects p ON t.project_id = p.id WHERE p.user_id = ");
    query_builder.push_bind(ctx.user.id);

    let mut count_query_builder = sqlx::QueryBuilder::new("SELECT COUNT(t.id) FROM tasks t JOIN projects p ON t.project_id = p.id WHERE p.user_id = ");
    count_query_builder.push_bind(ctx.user.id);

    if let Some(status) = params.status {
        query_builder.push(" AND t.status = ");
        query_builder.push_bind(status);
        count_query_builder.push(" AND t.status = ");
        count_query_builder.push_bind(status);
    }

    query_builder.push(" ORDER BY t.created_at DESC LIMIT ");
    query_builder.push_bind(limit as i64);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset as i64);

    let tasks: Vec<Task> = query_builder.build_query_as().fetch_all(&db).await?;

    let total_tasks: i64 = count_query_builder.build_query_scalar().fetch_one(&db).await?;

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
    let task = sqlx::query_as::<_, Task>(
        "SELECT t.* FROM tasks t JOIN projects p ON t.project_id = p.id WHERE t.id = $1 AND p.user_id = $2",
    )
    .bind(task_id)
    .bind(ctx.user.id)
    .fetch_optional(&db)
    .await?
    .ok_or(Error::TaskNotFound)?;

    Ok(Json(json!({ "status": "success", "data": { "task": task } })))
}

async fn update_task(
    ctx: Ctx,
    State(db): State<Db>,
    Path(task_id): Path<i64>,
    Json(payload): Json<UpdateTaskPayload>,
) -> Result<Json<Value>> {
    let task = sqlx::query_as::<_, Task>(
        "UPDATE tasks t SET title = COALESCE($1, t.title), description = COALESCE($2, t.description), status = COALESCE($3, t.status), due_date = COALESCE($4, t.due_date) FROM projects p WHERE t.id = $5 AND t.project_id = p.id AND p.user_id = $6 RETURNING t.*",
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.status)
    .bind(payload.due_date)
    .bind(task_id)
    .bind(ctx.user.id)
    .fetch_optional(&db)
    .await?
    .ok_or(Error::TaskUnauthorized)?;

    Ok(Json(json!({ "status": "success", "data": { "task": task } })))
}

async fn delete_task(
    ctx: Ctx,
    State(db): State<Db>,
    Path(task_id): Path<i64>,
) -> Result<Json<Value>> {
    let rows_affected = sqlx::query(
        "DELETE FROM tasks t USING projects p WHERE t.id = $1 AND t.project_id = p.id AND p.user_id = $2",
    )
    .bind(task_id)
    .bind(ctx.user.id)
    .execute(&db)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(Error::TaskUnauthorized);
    }

    Ok(Json(json!({ "status": "success", "message": "Task deleted" })))
}