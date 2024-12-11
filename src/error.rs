use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Debug, Display)]

//สร้าง enum ก่อนว่าเราจะใช้งานอะไรบ้าง
pub enum AppError {
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
impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(msg) => AppError::NotFound(msg),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

// implement error trait ของ Response
impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = match self {
            AppError::DatabaseError(message) => ErrorResponse {
                error: "DatabaseError".to_string(),
                message: message.clone(),
            },
            AppError::ValidationError(message) => ErrorResponse {
                error: "ValidationError".to_string(),
                message: message.clone(),
            },
            AppError::NotFound(message) => ErrorResponse {
                error: "NotFound".to_string(),
                message: message.clone(),
            },
            AppError::AuthenticationError(message) => ErrorResponse {
                error: "AuthenticationError".to_string(),
                message: message.clone(),
            },
            AppError::InternalServerError => ErrorResponse {
                error: "InternalServerError".to_string(),
                message: "An unexpected error occurred".to_string(),
            },
        };

        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) | AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}