use actix_web::{dev::Body, error::ResponseError, http::StatusCode, BaseHttpResponse};
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DieselError},
};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use libreauth::pass::ErrorCode as PassErrorCode;
use serde_json::{json, Map as JsonMap, Value as JsonValue};
use std::convert::From;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum Error {
    // 401
    #[error("Unauthorized: {}", .0)]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {}", .0)]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {}", .0)]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable Entity: {}", .0)]
    UnprocessableEntity(JsonValue),

    // 400
    #[error("Validation failed: {}", .0)]
    ValidationFailed(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,
}

// the ResponseError trait lets us convert errors to http responses with appropriate data
// https://actix.rs/docs/errors/
impl ResponseError for Error {
    fn error_response(&self) -> BaseHttpResponse<Body> {
        match *self {
            Error::Unauthorized(ref message) => {
                BaseHttpResponse::build(StatusCode::UNAUTHORIZED).finish()
            }
            Error::Forbidden(ref message) => {
                BaseHttpResponse::build(StatusCode::FORBIDDEN).finish()
            }
            Error::NotFound(ref message) => BaseHttpResponse::not_found(),
            Error::UnprocessableEntity(ref message) => {
                BaseHttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).finish()
            }
            Error::ValidationFailed(ref message) => {
                BaseHttpResponse::build(StatusCode::BAD_REQUEST).finish()
            }
            Error::InternalServerError => BaseHttpResponse::internal_server_error(),
        }
    }
}

impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        match error.kind() {
            JwtErrorKind::InvalidToken => Error::Unauthorized(json!({
                "error": "Token is invalid",
            })),
            JwtErrorKind::InvalidIssuer => Error::Unauthorized(json!({
                "error": "Issuer is invalid",
            })),
            _ => Error::Unauthorized(json!({
                "error": "An issue was found with the token provided",
            })),
        }
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::UnprocessableEntity(json!({ "error": message }));
                }
                Error::InternalServerError
            }
            DieselError::NotFound => {
                Error::NotFound(json!({ "error": "requested record was not found" }))
            }
            _ => Error::InternalServerError,
        }
    }
}

impl From<PoolError> for Error {
    fn from(_error: PoolError) -> Self {
        Error::InternalServerError
    }
}

impl From<PassErrorCode> for Error {
    fn from(_error: PassErrorCode) -> Self {
        Error::InternalServerError
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();

        // transforms errors into objects that err_map can take
        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors
                .iter()
                .map(|error| {
                    // dbg!(error) // <- Uncomment this if you want to see what error looks like
                    json!(error.message)
                })
                .collect();
            err_map.insert(field.to_string(), json!(errors));
        }

        Error::ValidationFailed(json!({
            "errors": err_map,
        }))
    }
}
