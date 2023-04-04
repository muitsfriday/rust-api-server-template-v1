use anyhow::Result;
use async_trait::async_trait;
use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

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
}

pub async fn create(store: &impl Store, data: Account) -> Result<Account> {
    let mut acc = data.clone();
    let r = store.save(&mut acc).await;
    Ok(data)
}
