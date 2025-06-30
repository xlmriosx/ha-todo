use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::model::Todo;

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub type ApiResponseTodo = ApiResponse<Todo>;
pub type ApiResponseVecTodo = ApiResponse<Vec<Todo>>;
pub type ApiResponseString = ApiResponse<String>;

impl ToSchema<'_> for ApiResponseTodo {
    fn schema() -> (&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>) {
        use utoipa::openapi::*;
        (
            "ApiResponseTodo",
            ObjectBuilder::new()
                .property(
                    "status",
                    ObjectBuilder::new()
                        .schema_type(SchemaType::String)
                        .example(Some(serde_json::json!("success")))
                )
                .property(
                    "data",
                    RefOr::Ref(Ref::from_schema_name("Todo"))
                )
                .property(
                    "error",
                    ObjectBuilder::new()
                        .schema_type(SchemaType::String)
                        .nullable(true)
                )
                .required("status")
                .into(),
        )
    }
}

impl ToSchema<'_> for ApiResponseVecTodo {
    fn schema() -> (&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>) {
        use utoipa::openapi::*;
        (
            "ApiResponseVecTodo",
            ObjectBuilder::new()
                .property(
                    "status",
                    ObjectBuilder::new()
                        .schema_type(SchemaType::String)
                        .example(Some(serde_json::json!("success")))
                )
                .property(
                    "data",
                    ArrayBuilder::new()
                        .items(RefOr::Ref(Ref::from_schema_name("Todo")))
                )
                .property(
                    "error",
                    ObjectBuilder::new()
                        .schema_type(SchemaType::String)
                        .nullable(true)
                )
                .required("status")
                .into(),
        )
    }
}

impl ToSchema<'_> for ApiResponseString {
    fn schema() -> (&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>) {
        use utoipa::openapi::*;
        (
            "ApiResponseString",
            ObjectBuilder::new()
                .property(
                    "status",
                    ObjectBuilder::new()
                        .schema_type(SchemaType::String)
                        .example(Some(serde_json::json!("success")))
                )
                .property(
                    "data",
                    ObjectBuilder::new()
                        .schema_type(SchemaType::String)
                        .nullable(true)
                )
                .property(
                    "error",
                    ObjectBuilder::new()
                        .schema_type(SchemaType::String)
                        .nullable(true)
                )
                .required("status")
                .into(),
        )
    }
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: "success".to_string(),
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            status: "error".to_string(),
            data: None,
            error: Some(message.to_string()),
        }
    }
}