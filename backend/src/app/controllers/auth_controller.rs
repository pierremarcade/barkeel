use crate::config::application::Config;
use crate::app::models::user::{ Credentials, Backend };
use std::sync::Arc;
use tera::{Context, Tera};
use axum::{ extract::State, response::{ IntoResponse, Redirect }, http::StatusCode, Form};
use crate::app::utils::response::Response;

type AuthSession = axum_login::AuthSession<Backend>;

//https://github.com/maxcountryman/axum-login/blob/main/examples/sqlite/src/web/app.rs
pub mod get {
    use super::*;
    pub async fn login(State(config): State<Arc<Config>>) -> impl IntoResponse {
        let tera: &Tera = &config.template;
        let mut tera = tera.clone();
        tera.add_raw_template("user/login.html", include_str!("../views/user/login.html")).unwrap();

        let rendered = tera.render("user/login.html", & Context::new()).unwrap();
        Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
    }
}
pub mod post {
    use super::*;
    pub async fn login(
    mut _auth_session: AuthSession, Form(_creds): Form<Credentials>

        ) -> impl IntoResponse {
        


            Redirect::to("/").into_response()
    }
}