use anyhow::Result;
use async_trait::async_trait;
use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use thiserror::Error;

#[derive(Debug, Default, Validate, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: Option<String>,
    pub username: String,
    pub password: String,
    pub email: String,
    pub info: Info,

    #[serde(with = "ts_milliseconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Validate, Serialize, Deserialize, Clone)]
pub struct Info {
    pub display_name: String,
    pub display_img: String,
}

#[async_trait]
pub trait Store {
    async fn save(&self, acc: &mut Account) -> Result<String>;
    async fn check_exists(&self, acc: &Account) -> Result<CheckExistsResult>;
}

pub enum CheckExistsResult {
    UserIsAvailable,
    EmailIsAlreadyExists,   
    UsernameIsAlreadyExists,
}

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("unexpected internal error")]
    Internal(String)
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("internal error {0}")]
    InternalError(String),
    #[error("username '{0}' is already taken")]
    UsernameIsAlreadyTaken(String),
    #[error("email '{0}' is already taken")]
    EmailIsAlreadyTaken(String),
}

pub async fn create(store: &impl Store, data: Account) -> Result<Account, ServiceError> {
    let mut acc = data.clone();
    let to_internal_err = |e: anyhow::Error| { ServiceError::InternalError(e.to_string()) };

    let is_exists = store.check_exists(&acc).await.map_err(to_internal_err)?;
    match is_exists {
        CheckExistsResult::UserIsAvailable => Ok(()),
        CheckExistsResult::EmailIsAlreadyExists => Err(ServiceError::EmailIsAlreadyTaken(data.email)),
        CheckExistsResult::UsernameIsAlreadyExists => Err(ServiceError::UsernameIsAlreadyTaken(data.username)),
    }?;

    let r = store.save(&mut acc).await.map_err(to_internal_err)?;
    acc.id = Some(r);

    Ok(acc)
}
