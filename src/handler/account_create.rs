use actix_web::{http::{header::ContentType, StatusCode}, web::{Data, Json}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::core::account as core_account;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RequestPayload {
    pub username: String,
    pub password: String,
    pub email: String,
    pub info: Info,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Info {
    pub display_name: String,
    pub display_img: String,
}

impl From<RequestPayload> for core_account::Account {
    fn from(payload: RequestPayload) -> Self {
        core_account::Account {
            username: payload.username,
            password: payload.password,
            email: payload.email,
            info: core_account::Info {
                display_name: payload.info.display_name,
                display_img: payload.info.display_img,
            },
            ..Default::default()
        }
    }
}

impl actix_web::error::ResponseError for core_account::ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(json!({
                "message": self.to_string(),
            }))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            core_account::ServiceError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            core_account::ServiceError::EmailIsAlreadyTaken(_) => StatusCode::CONFLICT,
            core_account::ServiceError::UsernameIsAlreadyTaken(_) => StatusCode::CONFLICT,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponsePayload {
    account: core_account::Account,
}

pub async fn handle<T: core_account::Store>(
    payload: Json<RequestPayload>,
    acc_store: Data<T>,
) -> actix_web::Result<impl Responder> {
    let data = payload.into_inner();
    let acc_store = acc_store.into_inner();
    let acc_store = acc_store.as_ref();

    let account = core_account::create(acc_store, data.into()).await?;
    
    Ok(Json(ResponsePayload { account }))
}
