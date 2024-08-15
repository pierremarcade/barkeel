use crate::config::application::Config;
use crate::app::models::user::User;
use crate::app::models::auth::Credentials;
use barkeel_lib::app::http::response::Response;
use crate::db::schema::users::dsl::*;
use std::sync::Arc;
use tera::{Context, Tera};
use axum::{extract::State, response::{IntoResponse, Response as AxumResponse}, http::{ HeaderMap, StatusCode }, Form, body::Body};
use barkeel_lib::session::CSRFManager;
use diesel::prelude::*;
use bcrypt::verify;
use crate::config::application::LOCALES;
use fluent_templates::Loader;

fn redirect_response(location: &str) -> AxumResponse {
    AxumResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", location)
        .body(Body::empty())
        .unwrap()
}

fn set_cookie_response(session_tok: &str) -> AxumResponse {
    AxumResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/")
        .header("Set-Cookie", format!("session_token={}", session_tok))
        .body(Body::empty())
        .unwrap()
}

pub mod get {
    use super::*;
    pub async fn login(headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
        let tera: &Tera = &config.template;
        let mut tera = tera.clone();
        tera.add_raw_template("login.html", include_str!("../views/login.html")).unwrap();
        let mut context = Context::new();
        let config_ref = config.as_ref();
        context.insert("data", &Credentials::build_create_form(config_ref, headers, "/login"));
        let rendered = tera.render("login.html", &context).unwrap();
        Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
    }

    pub async fn logout() -> impl IntoResponse {
        set_cookie_response("")
    }
}

pub mod post {
    use super::*;
    pub async fn login(State(config): State<Arc<Config>>, Form(creds): Form<Credentials>) -> impl IntoResponse {
        match users.filter(email.eq(creds.email)).first::<User>(&mut config.database.pool.get().unwrap()) {
            Ok(user) => {
                if let Err(_err) = verify(creds.password, &user.password) {
                    return redirect_response("/login");
                }
                let session_tok = new_session(&config, user.id).await;
                set_cookie_response(&session_tok)
            },
            _ => redirect_response("/login")
        }
    }
}

pub async fn new_session(
    config: &Config, 
    other_user_id: i32
) -> String {
    let csrf_manager = CSRFManager::new();
    let session_tok = csrf_manager.generate_csrf_token();
    let _updated_record: User = diesel::update(users)
            .filter(id.eq(other_user_id))
            .set(session_token.eq(session_tok.clone()))
            .get_result(&mut config.database.pool.get().unwrap())
            .unwrap_or_else(|_| { panic!("{}", LOCALES.lookup(&config.locale, "error_load").to_string()) });

    session_tok
}
