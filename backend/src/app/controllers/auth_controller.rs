use barkeel_lib::app::Config;
use crate::app::models::user::User;
use crate::app::models::auth::Credentials;
use barkeel_lib::app::http::response::Response;
use crate::db::schema::users::dsl::*;
use tera::{Context, Tera};
use axum::{extract::State, response::{IntoResponse, Response as AxumResponse}, http::{ HeaderMap, StatusCode }, Form, body::Body};
use barkeel_lib::session::CSRFManager;
use diesel::prelude::*;
use bcrypt::verify;
use crate::config::application::LOCALES;
use fluent_templates::Loader;
use crate::config::constants::USER_COOKIE_NAME;

fn redirect_response(location: &str) -> AxumResponse {
    AxumResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", location)
        .body(Body::empty())
        .unwrap()
}

fn set_cookie_response(session_tok: &str) -> AxumResponse {
    let cookie_value = if session_tok.is_empty() {
        format!("{}=; Expires=Thu, 01-Jan-1970 00:00:00 GMT; Max-Age=0", USER_COOKIE_NAME)
    } else {
        format!("{}={}", USER_COOKIE_NAME, session_tok)
    };
    AxumResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/")
        .header("Set-Cookie", cookie_value)
        .body(Body::empty())
        .unwrap()
}

pub mod get {
    use super::*;
    pub async fn login(headers: HeaderMap, State(config): State<Config>) -> impl IntoResponse {
        let tera: &Tera = &config.template;
        let mut tera = tera.clone();
        tera.add_raw_template("login.html", include_str!("../views/login.html")).unwrap();
        let mut context = Context::new();
        let config_ref = config.clone();
        context.insert("data", &Credentials::build_create_form(&config_ref, headers, "/login"));
        let rendered = tera.render("login.html", &context).unwrap();
        Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
    }

    pub async fn logout() -> impl IntoResponse {
        set_cookie_response("")
    }
}

pub mod post {
    use super::*;
    pub async fn login(State(config): State<Config>, headers: HeaderMap, Form(creds): Form<Credentials>) -> impl IntoResponse {
        match users.filter(email.eq(creds.email)).first::<User>(&mut config.database.pool.get().unwrap()) {
            Ok(user) => {
                if let Err(_err) = verify(creds.password, &user.password) {
                    return redirect_response("/login");
                }
                let session_tok = new_session(&config, user.id, headers).await;
                set_cookie_response(&session_tok)
            },
            _ => redirect_response("/login")
        }
    }
}

pub async fn new_session(
    config: &Config, 
    other_user_id: i32,
    headers: HeaderMap
) -> String {
    let csrf_manager = CSRFManager::new();
    let locale = crate::app::controllers::get_locale(headers, None);
    let session_tok = csrf_manager.generate_csrf_token();
    diesel::update(users)
        .filter(id.eq(other_user_id))
        .set(session_token.eq(session_tok.clone()))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Failed to update user record");

    session_tok
}
