use crate::config::application::Config;
use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Queryable, FormBuilder, Validate, Clone)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role_id: Option<i32>,
    pub session_token: Option<String>,

}
