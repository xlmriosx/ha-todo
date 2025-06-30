use serde::{Serialize, Deserialize};
use uuid::Uuid;
use utoipa::ToSchema;
use sqlx::{PgPool, FromRow};
use chrono::{DateTime, Utc};

pub struct AppState {
    pub db: PgPool,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, FromRow)]
#[schema(example = json!({
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Buy groceries",
    "completed": false,
    "created_at": "2023-01-01T00:00:00Z",
    "updated_at": "2023-01-01T00:00:00Z"
}))]
pub struct Todo {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "Buy groceries")]
    pub title: String,
    #[schema(example = false)]
    pub completed: bool,
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}