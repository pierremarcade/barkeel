use crate::config::application::Config;
use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Serialize, Deserialize, Identifiable, Queryable, FormBuilder, Selectable, Clone)]
#[diesel(table_name = crate::db::schema::articles)]
#[form_builder(label = title, id = id)]
pub struct Article {
    pub id: i32,
    pub title: String,
    #[form_builder_exclude]
    pub slug: String,
    #[form_builder(type_="textarea")]
    pub content: String,
    #[form_builder_exclude]
    pub published_at: NaiveDateTime,
    #[form_builder_exclude]
    pub author_id: Option<i32>,
    pub homepage: bool,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::articles)]
pub struct ArticleInsertValues {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published_at: NaiveDateTime,
    pub author_id: Option<i32>,
    pub homepage: bool,
}


#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::articles)]
pub struct ArticleUpdateValues {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub homepage: bool,
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Queryable)]
pub struct ArticleWithMenu{
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub homepage: bool,
    pub section_name: String
}

#[derive(Serialize, Deserialize)]
pub struct ArticleWithMenuAndMeta{
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub homepage: bool,
    pub section_name: String,
    pub description: Option<String>,
}

impl ArticleWithMenuAndMeta {
    pub fn new(result: (i32, String, String, String, bool, String, Option<String>)) -> Self {
        let ( other_id, other_title, other_slug, other_content, other_homepage, other_name, other_description) = result.clone();
        ArticleWithMenuAndMeta {
            id: other_id,
            title: other_title,
            slug: other_slug,
            content: other_content,
            homepage: other_homepage,
            section_name: other_name,
            description: other_description
        }
    }
}