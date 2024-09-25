use anyhow::Result;
use async_trait::async_trait;
use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use mongodb::{bson::oid, Collection};
use serde::{Deserialize, Serialize};
use crate::core::{account as core_account, account::CheckExistsResult};
use bson::{doc, Bson};
use log;

// Account struct represents the account data stored in the mongo collection
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub _id: Option<oid::ObjectId>,
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    pub user: UserInfo,

    #[serde(with = "ts_milliseconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub display_name: String,
    pub display_image: String,
}

impl From<core_account::Account> for Account {
    fn from(data: core_account::Account) -> Self {
        Self {
            _id: None,
            id: data.id,
            username: data.username,
            email: data.email,
            user: UserInfo {
                display_name: data.info.display_name,
                display_image: data.info.display_img,
            },
            created_at: data.created_at,
            updated_at: data.updated_at,
            deleted_at: data.deleted_at,
        }
    }
}


// Repo
pub struct Repo {
    coll: Collection<Account>,
}

impl Repo {
    pub fn new(coll: Collection<Account>) -> Self {
        Self { coll }
    }
}

#[async_trait]
impl core_account::Store for Repo {
    async fn save(&self, data: &mut core_account::Account) -> Result<String> {
        let now = Utc::now();
        data.updated_at = Some(now);
        if data.created_at.is_none() {
            data.created_at = Some(now);
        }

        let mut document = Account::from(data.clone());
        if document._id.is_none() {
            document._id = Some(oid::ObjectId::new());
        }

        let result = self.coll.insert_one(document).await;
        let insert_result = result.map_err(|e| { 
            core_account::StoreError::Internal(e.to_string())
        })?;

        if let Bson::ObjectId(ref id) = insert_result.inserted_id {
            data.id = Some(id.to_hex());
            let dd = oid::ObjectId::parse_str(id.to_hex());
            match dd {
                Ok(o) => log::info!("created: {}", o),
                Err(e) => log::error!("error: {}", e),
            }
        } else {
            return Err(core_account::StoreError::Internal("undefined inserted id".to_string()).into());
        }

        Ok(data.id.clone().unwrap_or_default())
    }

    async fn check_exists(&self, data: &core_account::Account) -> Result<CheckExistsResult> {
        let find_result = self.coll.find_one(doc! {
            "$or": [
                { "email": data.email.clone() },
                { "username": data.username.clone() },
            ]
        }).await?;
        
        if let Some(exists_data) = find_result {
            if exists_data.email == data.email {
                return Ok(CheckExistsResult::EmailIsAlreadyExists);
            }
            if exists_data.username == data.username {
                return Ok(CheckExistsResult::UsernameIsAlreadyExists);
            }
        };

        return Ok(CheckExistsResult::UserIsAvailable)
    }
}