use axum::{
    extract::{State, Path, Json, Query},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;
use utoipa::{ToSchema, IntoParams};
use validator::Validate;
use tracing::info;
use crate::{
    model::Todo, 
    response::ApiResponse, 
    model::AppState,
    error::AppError
};
use std::sync::Arc;

#[derive(Deserialize, ToSchema, Validate)]
#[schema(example = json!({
    "title": "Buy groceries",
    "completed": false
}))]
pub struct CreateTodo {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    #[schema(example = "Buy groceries")]
    title: String,
    #[schema(example = false)]
    completed: Option<bool>,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct PaginationQuery {
    #[schema(example = 1)]
    /// Page number (starts from 1)
    page: Option<u32>,
    #[schema(example = 10)]
    /// Number of items per page (max 100)
    limit: Option<u32>,
}

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = ApiResponseString)
    ),
    tag = "health"
)]
pub async fn health_check() -> impl IntoResponse {
    info!("Health check requested");
    (StatusCode::OK, Json(ApiResponse::<String>::success("Service is healthy".to_string())))
}

#[utoipa::path(
    post,
    path = "/api/v1/todos",
    request_body = CreateTodo,
    responses(
        (status = 201, description = "Todo created successfully", body = ApiResponseTodo),
        (status = 400, description = "Invalid input", body = ApiResponseString),
        (status = 500, description = "Database error", body = ApiResponseString)
    ),
    tag = "todos"
)]
pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(todo): Json<CreateTodo>,
) -> Result<impl IntoResponse, AppError> {
    // Validar entrada
    todo.validate()?;

    let result = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (title, completed)
        VALUES ($1, $2)
        RETURNING id, title, completed, created_at, updated_at
        "#
    )
    .bind(&todo.title)
    .bind(todo.completed.unwrap_or(false))
    .fetch_one(&state.db)
    .await?;

    info!("Todo created successfully with id: {}", result.id);
    Ok((StatusCode::CREATED, Json(ApiResponse::success(result))))
}

#[utoipa::path(
    get,
    path = "/api/v1/todos",
    params(PaginationQuery),
    responses(
        (status = 200, description = "List of todos retrieved successfully", body = ApiResponseVecTodo),
        (status = 500, description = "Database error", body = ApiResponseString)
    ),
    tag = "todos"
)]
pub async fn get_todos(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let page = pagination.page.unwrap_or(1).max(1);
    let limit = pagination.limit.unwrap_or(10).min(100).max(1);
    let offset = (page - 1) * limit;

    let todos = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, completed, created_at, updated_at
        FROM todos
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    info!("Retrieved {} todos (page: {}, limit: {})", todos.len(), page, limit);
    Ok((StatusCode::OK, Json(ApiResponse::success(todos))))
}

#[utoipa::path(
    get,
    path = "/api/v1/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo ID")
    ),
    responses(
        (status = 200, description = "Todo found", body = ApiResponseTodo),
        (status = 404, description = "Todo not found", body = ApiResponseString),
        (status = 500, description = "Database error", body = ApiResponseString)
    ),
    tag = "todos"
)]
pub async fn get_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, completed, created_at, updated_at
        FROM todos
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?;

    match todo {
        Some(todo) => {
            info!("Todo found with id: {}", id);
            Ok((StatusCode::OK, Json(ApiResponse::success(todo))))
        }
        None => {
            info!("Todo not found with id: {}", id);
            Err(AppError::NotFound)
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo ID")
    ),
    request_body = CreateTodo,
    responses(
        (status = 200, description = "Todo updated successfully", body = ApiResponseTodo),
        (status = 404, description = "Todo not found", body = ApiResponseString),
        (status = 400, description = "Invalid input", body = ApiResponseString),
        (status = 500, description = "Database error", body = ApiResponseString)
    ),
    tag = "todos"
)]
pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(todo): Json<CreateTodo>,
) -> Result<impl IntoResponse, AppError> {
    // Validar entrada
    todo.validate()?;

    let updated_todo = sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos 
        SET title = $1, completed = $2, updated_at = NOW()
        WHERE id = $3
        RETURNING id, title, completed, created_at, updated_at
        "#
    )
    .bind(&todo.title)
    .bind(todo.completed.unwrap_or(false))
    .bind(id)
    .fetch_optional(&state.db)
    .await?;

    match updated_todo {
        Some(todo) => {
            info!("Todo updated successfully with id: {}", id);
            Ok((StatusCode::OK, Json(ApiResponse::success(todo))))
        }
        None => {
            info!("Todo not found for update with id: {}", id);
            Err(AppError::NotFound)
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo ID")
    ),
    responses(
        (status = 200, description = "Todo deleted successfully", body = ApiResponseString),
        (status = 404, description = "Todo not found", body = ApiResponseString),
        (status = 500, description = "Database error", body = ApiResponseString)
    ),
    tag = "todos"
)]
pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM todos 
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(&state.db)
    .await?;

    if result.rows_affected() > 0 {
        info!("Todo deleted successfully with id: {}", id);
        Ok((StatusCode::OK, Json(ApiResponse::<String>::success("Todo deleted successfully".to_string()))))
    } else {
        info!("Todo not found for deletion with id: {}", id);
        Err(AppError::NotFound)
    }
}