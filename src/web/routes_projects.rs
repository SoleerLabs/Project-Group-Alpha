use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::errors::{Error, Result}; // errors.rs is at crate root
use utoipa::OpenApi;
// All these are siblings within the 'web' module
use crate::ctx::Ctx; // Correct path to Ctx
use super::db::Db;   // Correct path to Db
use super::projects::{ // Correct path to project models
    CreateProjectPayload, Project, ProjectListQueryParams, UpdateProjectPayload,
};
pub fn routes(db: Db) -> Router {
    Router::new()
        .route("/projects", post(create_project).get(list_projects))
        .route(
            "/projects/{id}",
            get(get_project_by_id)
                .put(update_project)
                .delete(delete_project),
        )
        .with_state(db)
}

// POST /projects
// Body: { name, description }
// Creates a project for the logged-in user.
#[utoipa::path(
    post,
    path = "/projects",
    request_body = CreateProjectPayload, // Refers to the struct defined with ToSchema
    responses(
        (status = 201, description = "Project created successfully", body = Project), // Assuming Project is returned
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt_token" = [])) // Marks this as secured by JWT
)]
async fn create_project(
    Ctx { user: ctx_user }: Ctx, // FIXED: Destructure Ctx using struct pattern
    State(db): State<Db>,
    Json(payload): Json<CreateProjectPayload>,
) -> Result<Json<Value>> {
    let project = sqlx::query_as!(
        Project,
        "INSERT INTO projects (user_id, name, description) VALUES ($1, $2, $3) RETURNING id, user_id, name, description, created_at, updated_at",
        ctx_user.id, // Use ctx_user.id
        payload.name,
        payload.description
    )
    .fetch_one(&db)
    .await?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "project": project
        }
    })))
}

// GET /projects
// Query params: ?page=1&limit=10
// Lists all projects owned by the user with pagination.

async fn list_projects(
    Ctx { user: ctx_user }: Ctx, // FIXED
    State(db): State<Db>,
    Query(params): Query<ProjectListQueryParams>,
) -> Result<Json<Value>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let projects = sqlx::query_as!(
        Project,
        "SELECT id, user_id, name, description, created_at, updated_at
         FROM projects
         WHERE user_id = $1
         ORDER BY created_at DESC
         LIMIT $2 OFFSET $3",
        ctx_user.id, // Use ctx_user.id
        limit as i64,
        offset as i64
    )
    .fetch_all(&db)
    .await?;

    let total_projects = sqlx::query_scalar!(
        "SELECT COUNT(id) FROM projects WHERE user_id = $1",
        ctx_user.id // Use ctx_user.id
    )
    .fetch_one(&db)
    .await?
    .unwrap_or(0); // COUNT returns Option<i64>

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

// GET /projects/:id
// Fetch a specific project details (only if user owns it).
async fn get_project_by_id(
    Ctx { user: ctx_user }: Ctx, // FIXED
    State(db): State<Db>,
    Path(project_id): Path<i64>,
) -> Result<Json<Value>> {
    let project = sqlx::query_as!(
        Project,
        "SELECT id, user_id, name, description, created_at, updated_at
         FROM projects
         WHERE id = $1",
        project_id
    )
    .fetch_optional(&db)
    .await?
    .ok_or(Error::ProjectNotFound)?;

    // Ensure the project belongs to the authenticated user
    if project.user_id != ctx_user.id { // Use ctx_user.id
        return Err(Error::ProjectUnauthorized);
    }

    Ok(Json(json!({
        "status": "success",
        "data": {
            "project": project
        }
    })))
}

// PUT /projects/:id
// Update project info.
async fn update_project(
    Ctx { user: ctx_user }: Ctx, // FIXED
    State(db): State<Db>,
    Path(project_id): Path<i64>,
    Json(payload): Json<UpdateProjectPayload>,
) -> Result<Json<Value>> {
    let updated_project = sqlx::query_as!(
        Project,
        "UPDATE projects
         SET
             name = COALESCE($1, name),
             description = COALESCE($2, description),
             updated_at = CURRENT_TIMESTAMP
         WHERE id = $3 AND user_id = $4
         RETURNING id, user_id, name, description, created_at, updated_at",
        payload.name,
        payload.description,
        project_id,
        ctx_user.id // Use ctx_user.id
    )
    .fetch_optional(&db)
    .await?
    .ok_or(Error::ProjectUnauthorized)?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "project": updated_project
        }
    })))
}

// DELETE /projects/:id
// Deletes project and cascades delete tasks.
async fn delete_project(
    Ctx { user: ctx_user }: Ctx, // FIXED
    State(db): State<Db>,
    Path(project_id): Path<i64>,
) -> Result<Json<Value>> {
    let rows_affected = sqlx::query!(
        "DELETE FROM projects WHERE id = $1 AND user_id = $2",
        project_id,
        ctx_user.id
    )
    .execute(&db)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(Error::ProjectUnauthorized);
    }

    Ok(Json(json!({
        "status": "success",
        "message": "Project and associated tasks deleted successfully"
    })))
}