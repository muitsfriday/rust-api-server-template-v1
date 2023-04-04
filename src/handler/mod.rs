#![allow(dead_code)]
#![allow(unused_variables)]
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

pub mod account_create;

#[derive(Debug, Display, Error)]
pub enum APIError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
