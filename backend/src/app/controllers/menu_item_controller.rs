use crate::config::application::Config;
use crate::app::models::menu_item::{ MenuItem, MenuItemForm, MenuItemValues };
use crate::app::models::user::User;
use crate::db::schema::menu_items::dsl::*;
use diesel::prelude::*;
use std::sync::Arc;
use tera::Tera;
use axum::{  Extension, extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use crate::app::controllers::{ get_content_type, is_csrf_token_valid, error_controller, prepare_tera_context };
use crate::app::middlewares::auth::AuthState;
use barkeel_lib::app::pagination::{ PaginationQuery, Pagination, PaginationTrait };
use barkeel_lib::app::http::response::Response;
use validator::{Validate, ValidationErrors};
use crate::crud;
use inflector::Inflector;


fn insert_values(payload: MenuItemForm, _current_user: User) -> MenuItemValues {
    MenuItemValues {
        menu_id: payload.menu_id,
        article_id: payload.article_id,
        label: payload.label,
        position: payload.position,
    }
}

fn update_values(payload: MenuItemForm, _current_user: User) -> MenuItemValues {
    MenuItemValues {
        menu_id: payload.menu_id,
        article_id: payload.article_id,
        label: payload.label,
        position: payload.position,
    }
}

crud!(menu_items, MenuItem, MenuItemForm);
