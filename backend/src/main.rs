use axum::Router;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use model::AppState;
use routes::app_routes;
use sqlx::PgPool;
use dotenvy::dotenv;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use config::Config;

mod routes;
mod handler;
mod response;
mod model;
mod config;
mod error;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::create_todo,
        handler::get_todos,
        handler::get_todo,
        handler::update_todo,
        handler::delete_todo,
        handler::health_check
    ),
    components(
        schemas(
            model::Todo,
            handler::CreateTodo,
            handler::PaginationQuery,
            response::ApiResponseTodo,
            response::ApiResponseVecTodo,
            response::ApiResponseString
        )
    ),
    tags(
        (name = "todos", description = "Todo management API"),
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "Todo API",
        version = "1.0.0",
        description = "A simple Todo API built with Rust and Axum with PostgreSQL",
        contact(
            name = "API Support",
            email = "support@todoapi.com"
        )
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_env_filter("backend=debug,tower_http=debug")
        .init();

    // Cargar variables de entorno
    dotenv().ok();

    let config = Config::from_env()
        .expect("Failed to load configuration from environment variables");

    tracing::info!("Starting Todo API server...");
    tracing::debug!("Database URL: {}", config.database_url);

    // Crear pool de conexiones
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Ejecutar migraciones
    tracing::info!("🔄 Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    tracing::info!("✅ Migrations completed successfully");

    let state = Arc::new(AppState {
        db: pool,
    });

    let app = Router::new()
        .nest("/api/v1/todos", app_routes())
        .route("/api/v1/health", axum::routing::get(handler::health_check))
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let bind_address = format!("{}:{}", config.server_host, config.server_port);
    
    tracing::info!("🚀 Server starting on http://{}", bind_address);
    tracing::info!("📚 Swagger UI available at http://{}/swagger-ui", bind_address);
    tracing::info!("🗄️ Database connected successfully");
    tracing::info!("🏥 Health check available at http://{}/api/v1/health", bind_address);

    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}