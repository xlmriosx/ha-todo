use axum::{routing::{get, post, put, delete}, Router};
use crate::handler::{create_todo, get_todos, get_todo, update_todo, delete_todo};
use crate::model::AppState;
use std::sync::Arc;

pub fn app_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_todo))
        .route("/", get(get_todos))
        .route("/:id", get(get_todo))
        .route("/:id", put(update_todo))
        .route("/:id", delete(delete_todo))
}