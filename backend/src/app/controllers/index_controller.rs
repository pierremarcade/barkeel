
use axum::{ Extension, extract::State, response:: { Html, IntoResponse } };
use tera::Tera;
use barkeel_lib::app::Config;
use crate::app::models::auth::AuthState;

pub async fn index(Extension(current_user): Extension<AuthState>, State(config): State<Config>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("index.html", include_str!("../views/index.html")).unwrap();
    let context = crate::app::controllers::prepare_tera_context(current_user).await;
    let rendered = tera.render("index.html", &context).unwrap();
    Html(rendered)
}