use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use tokio::task;
use password_auth::verify_password;
use crate::db::schema::users::dsl::*;

#[derive(Serialize, Deserialize, Queryable, FormBuilder, Clone, Debug)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role_id: Option<i32>,
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password.as_bytes()
    }
}

#[derive(Clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Backend {
    config: Config,
}

impl Backend {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user_result = users.filter(email.eq(creds.email)).first::<User>(&mut self.config.database.pool.get().unwrap());

        match user_result {
            Ok(user) => {
                if verify_password(creds.password, &user.password).is_ok() {
                    Ok(Some(user))
                } else {
                    Ok(None) 
                }
            },
            Err(_) => Ok(None), 
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user_result = users.find(user_id).first::<User>(&mut self.config.database.pool.get().unwrap());

        match user_result {
            Ok(user) => Ok(Some(user)),
            Err(_) => Ok(None),
        }
    }
}