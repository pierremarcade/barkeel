use barkeel_lib::app::Config;
use crate::app::models::menu_item::{ MenuItem, MenuItemForm, MenuItemValues };
use crate::app::models::user::User;
use crate::db::schema::menu_items::dsl::*;
use diesel::prelude::*;
use tera::Tera;
use axum::{  Extension, extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use crate::app::controllers::CrudViewTrait;
use crate::app::models::auth::AuthState;
use barkeel_lib::app::pagination::{ RequestQuery, Pagination, PaginationTrait };
use barkeel_lib::app::http::response::Response;
use validator::{Validate, ValidationErrors};
use barkeel_lib::crud;
use inflector::Inflector;
use crate::config::application::LOCALES;
use fluent_templates::Loader;
use std::collections::HashMap;

type CrudModel = MenuItem;
type CrudForm = MenuItemForm;

pub struct MenuItemViewCrud;
impl CrudViewTrait for MenuItemViewCrud {}

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

crud!(menu_items, MenuItemViewCrud);
