use barkeel_lib::app::Config;
use diesel::prelude::*;
use barkeel_derives::{FormBuilder, OrderBy};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::db::schema::article_menus::dsl::article_menus;
#[cfg(feature = "postgres")]
use barkeel_lib::database::postgres::DB;
#[cfg(feature = "mysql")]
use barkeel_lib::database::mysql::DB;
#[cfg(feature = "sqlite")]
use barkeel_lib::database::sqlite::DB;

#[derive(Serialize, Deserialize, Queryable, FormBuilder, Validate, Clone, OrderBy)]
#[diesel(table_name = crate::db::schema::article_menus)]
pub struct ArticleMenu {
    pub article_id: i32,
    pub menu_id: i32,

}
