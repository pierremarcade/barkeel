use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Queryable, FormBuilder, Clone)]
#[diesel(table_name = crate::db::schema::article_metas)]
#[diesel(belongs_to(Article))]
pub struct ArticleMeta{
    pub article_id: i32,
    pub key: Option<String>,
    pub value: Option<String>,

}
