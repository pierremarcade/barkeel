use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Queryable, FormBuilder, Clone)]
#[diesel(table_name = crate::db::schema::article_seos)]
pub struct ArticleSeo {
    pub article_id: i32,
    pub title_seo: Option<String>,
    pub description_seo: Option<String>,
    pub keywords_seo: Option<String>,

}
