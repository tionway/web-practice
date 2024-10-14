use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt::{self, Debug};

#[derive(Debug, Serialize, Clone)]
pub enum TutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl TutorError {
    fn error_response(&self) -> String {
        match self {
            TutorError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            TutorError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            TutorError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            TutorError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for TutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            TutorError::DBError(_) | TutorError::ActixError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            TutorError::NotFound(_) => StatusCode::NOT_FOUND,
            TutorError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for TutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<actix_web::error::Error> for TutorError {
    fn from(err: actix_web::error::Error) -> Self {
        TutorError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for TutorError {
    fn from(err: SQLxError) -> Self {
        TutorError::DBError(err.to_string())
    }
}
