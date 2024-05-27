use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable, FormBuilder, Selectable, Clone)]
#[diesel(table_name = crate::db::schema::articles)]
#[form_builder(label = title, id = id)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    #[form_builder(type_="textarea")]
    pub content: String,
    #[exclude]
    pub published_at: NaiveDateTime,
    #[exclude]
    pub author_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::articles)]
pub struct ArticleMenu {
    pub id: i32,
    pub slug: String,
}
