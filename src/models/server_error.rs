use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde_json::json;

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "{}", message)]
    UserError { message: String },
    #[display(fmt = "{}", message)]
    InternalError { message: String },
}
impl ServerError {
    /// Get `name` of the error or `type` of error
    fn name(&self) -> String {
        match self {
            ServerError::InternalError { message } => "InternalError".to_string(),
            ServerError::UserError { message } => "UserError".to_string(),
        }
    }
    /// Get Error Message
    fn message(&self) -> String {
        match self {
            ServerError::InternalError { message } => message.to_string(),
            ServerError::UserError { message } => message.to_string(),
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = json!( {
            "code": status_code.as_u16(),
            "error": self.name(),
            "message": self.message(),
        });
        HttpResponse::build(status_code).json(error_response)
    }
}

pub fn map_to_server_error<T: ResponseError>(e: T) -> ServerError {
    ServerError::InternalError {
        message: e.to_string(),
    }
}
