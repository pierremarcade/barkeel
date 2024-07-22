use crate::config::application::Config;
use crate::app::models::user::{ User, UserForm, UserValues };
use crate::db::schema::users::dsl::*;
use crate::app::controllers::{ get_content_type, is_csrf_token_valid, error_controller, prepare_tera_context };
use crate::app::middlewares::auth::AuthState;
use crate::{ render_json, render_form, crud };
use barkeel_lib::app::pagination::{ PaginationQuery, Pagination, PaginationTrait };
use barkeel_lib::app::http::response::Response;
use diesel::prelude::*;
use std::sync::Arc;
use tera::Tera;
use axum::{  Extension, extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use validator::{Validate, ValidationErrors};
use inflector::Inflector;

fn insert_values(payload: UserForm) -> UserValues {
    UserValues {
        name: payload.name,
        email: payload.email,
        password: payload.password,
        role_id: payload.role_id,
        session_token: payload.session_token,
    }
}

fn update_values(payload: UserForm) -> UserValues {
    UserValues {
        name: payload.name,
        email: payload.email,
        password: payload.password,
        role_id: payload.role_id,
        session_token: payload.session_token,
    }
}

crud!(users, User, UserForm);
