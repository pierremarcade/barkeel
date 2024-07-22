use crate::config::application::Config;
use crate::app::models::menu::{ Menu, MenuForm, MenuValues };
use crate::db::schema::menus::dsl::*;
use crate::app::controllers::{ get_content_type, is_csrf_token_valid, error_controller, prepare_tera_context };
use crate::app::middlewares::auth::AuthState;
use crate::crud;
use barkeel_lib::app::pagination::{ PaginationQuery, Pagination, PaginationTrait };
use barkeel_lib::app::http::response::Response;
use diesel::prelude::*;
use std::sync::Arc;
use tera::Tera;
use axum::{  Extension, extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use validator::{Validate, ValidationErrors};
use inflector::Inflector;

fn insert_values(payload: MenuForm) -> MenuValues {
    MenuValues {
        name: payload.name,
    }
}

fn update_values(payload: MenuForm) -> MenuValues {
    MenuValues {
        name: payload.name,
    }
}

crud!(menus, Menu, MenuForm);