use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};

use super::APIError;
use crate::core::account;

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

impl From<RequestPayload> for account::Account {
    fn from(payload: RequestPayload) -> Self {
        account::Account {
            username: payload.username,
            password: payload.password,
            email: payload.email,
            info: account::Info {
                display_name: payload.info.display_name,
                display_img: payload.info.display_img,
            },
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponsePayload {
    data: account::Account,
}

pub async fn handle(
    payload: Json<RequestPayload>,
    acc_store: Data<impl account::Store>,
) -> Result<Json<ResponsePayload>, APIError> {
    let data = payload.into_inner();
    let acc_store = acc_store.into_inner();
    let acc_store = acc_store.as_ref();
    println!("{:#?}", data);

    let res = account::create(acc_store, data.into()).await.unwrap();

    Ok(Json(ResponsePayload { data: res }))
}
