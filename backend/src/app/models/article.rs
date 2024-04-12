use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable, FormBuilder, Clone)]
#[diesel(table_name = crate::db::schema::articles)]
pub struct Article {
    pub id: i32,
    pub title: String,
    #[form_builder(type_="textarea")]
    pub content: String,
    pub published_at: NaiveDateTime,
    pub author_id: Option<i32>,
}
