use crate::config::application::Config;
use crate::app::models::user::User;
use crate::app::models::session::Session;
use crate::app::models::auth::Credentials;
use std::sync::Arc;
use tera::{Context, Tera};
use axum::{extract::State, response::{IntoResponse, Response as AxumResponse}, http::StatusCode, Form, body::Body};
use crate::app::utils::response::Response;
use barkeel_lib::session::CSRFManager;
use diesel::prelude::*;
use crate::db::schema::users::dsl::*;
use crate::db::schema::sessions::dsl::*;
use bcrypt::verify;

fn redirect_response(location: &str) -> AxumResponse {
    AxumResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", location)
        .body(Body::empty())
        .unwrap()
}

fn set_cookie_response(session_tok: &str, max_age: &str) -> AxumResponse {
    AxumResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/")
        .header("Set-Cookie", format!("unique_id={}; Max-Age={}", session_tok, max_age))
        .body(Body::empty())
        .unwrap()
}

pub mod get {
    use super::*;
    pub async fn login(State(config): State<Arc<Config>>) -> impl IntoResponse {
        let tera: &Tera = &config.template;
        let mut tera = tera.clone();
        tera.add_raw_template("login.html", include_str!("../views/login.html")).unwrap();
        let rendered = tera.render("login.html", &Context::new()).unwrap();
        Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
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
                set_cookie_response(&session_tok, "999999")
            },
            _ => redirect_response("/login")
        }
    }
}

pub async fn logout_response() -> impl axum::response::IntoResponse {
    set_cookie_response("", "0")
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
