use barkeel_lib::app::Config;
use crate::app::models::user::{ User, UserForm, UserValues };
use crate::db::schema::users::dsl::*;
use crate::app::controllers::CrudViewTrait;
use crate::app::models::auth::AuthState;
use barkeel_lib::crud;
use barkeel_lib::app::pagination::{ RequestQuery, Pagination, PaginationTrait };
use barkeel_lib::app::http::response::Response;
use diesel::prelude::*;
use tera::Tera;
use axum::{  Extension, extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use validator::{Validate, ValidationErrors};
use inflector::Inflector;
use crate::config::application::LOCALES;
use fluent_templates::Loader;
use std::collections::HashMap;

type CrudModel = User;
type CrudForm = UserForm;

pub struct UserView;
impl CrudViewTrait for UserView {}

fn insert_values(payload: UserForm, _current_user: User) -> UserValues {
    UserValues {
        name: payload.name,
        email: payload.email,
        password: payload.password,
        role_id: payload.role_id,
        session_token: payload.session_token,
    }
}

fn update_values(payload: UserForm, _current_user: User) -> UserValues {
    UserValues {
        name: payload.name,
        email: payload.email,
        password: payload.password,
        role_id: payload.role_id,
        session_token: payload.session_token,
    }
}

crud!(users, UserView);