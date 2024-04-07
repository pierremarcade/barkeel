use diesel::prelude::*;
use crate::app::models::user::User;


#[derive(Queryable, Selectable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::db::schema::sessions)]
#[diesel(belongs_to(User))]
pub struct Session {
    pub session_token: String,
    pub user_id: Option<i32>,

}
