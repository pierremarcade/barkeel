use barkeel_lib::app::Config;
use diesel::prelude::*;
use barkeel_derives::{FormBuilder, OrderBy};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::db::schema::users::dsl::users;
#[cfg(feature = "postgres")]
use barkeel_lib::database::postgres::DB;
#[cfg(feature = "mysql")]
use barkeel_lib::database::mysql::DB;
#[cfg(feature = "sqlite")]
use barkeel_lib::database::sqlite::DB;

#[derive(Serialize, Deserialize, Queryable, FormBuilder, Validate, Clone, OrderBy)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role_id: Option<i32>,
    pub session_token: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::users)]
pub struct UserValues {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role_id: Option<i32>,
    pub session_token: Option<String>,
}
