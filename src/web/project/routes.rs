use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

use crate::ctx::Ctx;
use crate::errors::{Error, Result};
use crate::web::db::Db;
use crate::web::project::{
    CreateProjectPayload, Project, ProjectListQueryParams, UpdateProjectPayload,
};

pub fn routes(db: Db) -> Router {
    Router::new()
        .route("/projects", post(create_project).get(list_projects))
        .route(
            "/projects/{id}",
            get(get_project_by_id).put(update_project).delete(delete_project),
        )
        .with_state(db)
}

async fn create_project(
    ctx: Ctx,
    State(db): State<Db>,
    Json(payload): Json<CreateProjectPayload>,
) -> Result<Json<Value>> {
    let project = sqlx::query_as::<_, Project>(
        "INSERT INTO projects (user_id, name, description) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(ctx.user.id)
    .bind(payload.name)
    .bind(payload.description)
    .fetch_one(&db)
    .await?;

    Ok(Json(json!({ "status": "success", "data": { "project": project } })))
}

async fn list_projects(
    ctx: Ctx,
    State(db): State<Db>,
    Query(params): Query<ProjectListQueryParams>,
) -> Result<Json<Value>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let projects: Vec<Project> = sqlx::query_as(
        "SELECT * FROM projects WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
    )
    .bind(ctx.user.id)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(&db)
    .await?;

    let total_projects: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects WHERE user_id = $1")
        .bind(ctx.user.id)
        .fetch_one(&db)
        .await?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "projects": projects,
            "pagination": {
                "total": total_projects,
                "page": page,
                "limit": limit,
                "total_pages": (total_projects as f64 / limit as f64).ceil() as u32
            }
        }
    })))
}

async fn get_project_by_id(
    ctx: Ctx,
    State(db): State<Db>,
    Path(project_id): Path<i64>,
) -> Result<Json<Value>> {
    let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_optional(&db)
        .await?
        .ok_or(Error::ProjectNotFound)?;

    if project.user_id != ctx.user.id {
        return Err(Error::ProjectUnauthorized);
    }

    Ok(Json(json!({ "status": "success", "data": { "project": project } })))
}

async fn update_project(
    ctx: Ctx,
    State(db): State<Db>,
    Path(project_id): Path<i64>,
    Json(payload): Json<UpdateProjectPayload>,
) -> Result<Json<Value>> {
    let project = sqlx::query_as::<_, Project>(
        "UPDATE projects SET name = COALESCE($1, name), description = COALESCE($2, description) WHERE id = $3 AND user_id = $4 RETURNING *",
    )
    .bind(payload.name)
    .bind(payload.description)
    .bind(project_id)
    .bind(ctx.user.id)
    .fetch_optional(&db)
    .await?
    .ok_or(Error::ProjectUnauthorized)?;

    Ok(Json(json!({ "status": "success", "data": { "project": project } })))
}

async fn delete_project(
    ctx: Ctx,
    State(db): State<Db>,
    Path(project_id): Path<i64>,
) -> Result<Json<Value>> {
    let rows_affected = sqlx::query("DELETE FROM projects WHERE id = $1 AND user_id = $2")
        .bind(project_id)
        .bind(ctx.user.id)
        .execute(&db)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(Error::ProjectUnauthorized);
    }

    Ok(Json(json!({ "status": "success", "message": "Project deleted" })))
}