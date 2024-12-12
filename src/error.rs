use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Debug, Display)]

//สร้าง enum ก่อนว่าเราจะใช้งานอะไรบ้าง
pub enum ApiError {
    #[display("Database error: {}", _0)]
    DatabaseError(String),

    #[display("Validation error: {}", _0)]
    ValidationError(String),

    #[display("Not found: {}", _0)]
    NotFound(String),

    #[display("Authentication error: {}", _0)]
    AuthenticationError(String),

    #[display("Internal server error")]
    InternalServerError,
}

// implement error trait ของ DB error
impl From<DbErr> for ApiError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(msg) => ApiError::NotFound(msg),
            _ => ApiError::DatabaseError(err.to_string()),
        }
    }
}

// implement error trait ของ Response
impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let error_response = match self {
            ApiError::DatabaseError(message) => ErrorResponse {
                error: "DatabaseError".to_string(),
                message: message.clone(),
            },
            ApiError::ValidationError(message) => ErrorResponse {
                error: "ValidationError".to_string(),
                message: message.clone(),
            },
            ApiError::NotFound(message) => ErrorResponse {
                error: "NotFound".to_string(),
                message: message.clone(),
            },
            ApiError::AuthenticationError(message) => ErrorResponse {
                error: "AuthenticationError".to_string(),
                message: message.clone(),
            },
            ApiError::InternalServerError => ErrorResponse {
                error: "InternalServerError".to_string(),
                message: "An unexpected error occurred".to_string(),
            },
        };

        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::DatabaseError(_) | ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}