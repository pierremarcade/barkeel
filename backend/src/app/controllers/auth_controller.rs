use crate::config::application::Config;
use crate::app::models::user::User;
use crate::app::models::session::Session;
use crate::app::models::auth::{Credentials, CredentialsForm};
use std::sync::Arc;
use tera::{Context, Tera};
use axum::{ extract::State, response::{ IntoResponse, Redirect, Response as AxumResponse }, http::{ HeaderMap, StatusCode}, Form, body::Body};
use crate::app::utils::response::Response;
use barkeel_lib::session::CSRFManager;
use diesel::prelude::*;
use crate::db::schema::users::dsl::*;
use crate::db::schema::sessions::dsl::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::app::controllers::error_controller;

pub mod get {
    use super::*;
    pub async fn login(headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
        let tera: &Tera = &config.template;
        let mut tera = tera.clone();
        tera.add_raw_template("login.html", include_str!("../views/login.html")).unwrap();

        let mut context = Context::new();
        let config_ref = config.as_ref();
        context.insert("data",&CredentialsForm::new().build_form(config_ref, headers, "/login"));
    
        let rendered = tera.render("login.html", &context).unwrap();
        Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
    }
}
pub mod post {
    use super::*;
    pub async fn login(State(config): State<Arc<Config>>, Form(creds): Form<Credentials>) -> impl IntoResponse {
        match users.filter(email.eq(creds.email)).first::<User>(&mut config.database.pool.get().unwrap()) {
            Ok(user) => {
                if let Err(_err) = verify(creds.password, &user.password) {
                    AxumResponse::builder()
                    .header("Location", "/login")
                    .body(Body::empty())
                    .unwrap();
                }
                let session_tok = new_session(&config, user.id).await;

                AxumResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/")
        .header("Set-Cookie", format!("unique_id={}; Max-Age=999999", session_tok.as_str()))
        .body(Body::empty())
        .unwrap()
            },
            _ => {
                AxumResponse::builder()
                    .header("Location", "/login")
                    .body(Body::empty())
                    .unwrap()
            }
        }
    }
}

pub async fn logout_response() -> impl axum::response::IntoResponse {
    AxumResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/")
        .header("Set-Cookie", "unique_id=_; Max-Age=0")
        .body(Body::empty())
        .unwrap()
}

pub async fn new_session(
    config: &Config, 
    other_user_id: i32
) -> String {
    let csrf_manager = CSRFManager::new();

    let session_tok = csrf_manager.generate_csrf_token();

    let _inserted_record: Session = diesel::insert_into(sessions)
        .values((session_token.eq(session_tok.clone()), user_id.eq(other_user_id)))
        .get_result(&mut config.database.pool.get().unwrap())
        .expect("Error inserting data");

    session_tok
}

fn set_cookie(session_tok: &str) -> impl IntoResponse {
    AxumResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/")
        .header("Set-Cookie", format!("unique_id={}; Max-Age=999999", session_tok))
        .body(Body::empty())
        .unwrap()
}