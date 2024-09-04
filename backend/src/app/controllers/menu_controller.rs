use barkeel_lib::app::Config;
use crate::app::models::menu::{ Menu, MenuForm, MenuValues };
use crate::app::models::user::User;
use crate::db::schema::menus::dsl::*;
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

type CrudModel = Menu;
type CrudForm = MenuForm;

pub struct MenuView;

impl CrudViewTrait for MenuView {}

fn insert_values(payload: MenuForm, _current_user: User) -> MenuValues {
    MenuValues {
        name: payload.name,
    }
}

fn update_values(payload: MenuForm, _current_user: User) -> MenuValues {
    MenuValues {
        name: payload.name,
    }
}

crud!(menus, MenuView);