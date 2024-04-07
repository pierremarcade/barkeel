use crate::config::application::Config;
use crate::app::models::user::User;
use axum::{
    middleware::Next,
    extract::Request, 
    http::StatusCode,  
    body::Body,
    response::Response
};
use std::sync::Arc;
use crate::db::schema::users::dsl::{users};
use crate::db::schema::sessions::dsl::{sessions, session_token};
use diesel::prelude::*;
const USER_COOKIE_NAME: &str = "unique_id";

#[derive(Clone)]
pub(crate) struct AuthState(Option<(String, Option<User>, Arc<Config>)>);

pub(crate) async fn auth(
    config: Arc<Config>,
    mut request: Request, next: Next,
) -> axum::response::Response {
    let session_tok = request
        .headers()
        .get_all("Cookie")
        .iter()
        .filter_map(|cookie| {
            cookie
                .to_str()
                .ok()
                .and_then(|cookie| cookie.parse::<cookie::Cookie>().ok())
        })
        .find_map(|cookie| {
            println!("{}", cookie.name());
            (cookie.name() == USER_COOKIE_NAME).then(move || cookie.value().to_owned())
        })
        .and_then(|cookie_value| cookie_value.parse::<String>().ok());

    let is_logged_in = session_tok.is_some();

    // Récupérer le chemin de la requête
    let path = request.uri().path();

    // Rediriger en fonction de l'état de connexion et du chemin de la requête
    if is_logged_in && path == "/login" {
        return Response::builder()
            .status(StatusCode::FOUND)
            .header("Location", "/")
            .body(Body::empty())
            .unwrap();
    } else if !is_logged_in && path != "/login" && !path.starts_with("/public") {
        // Supposons que toutes les routes commençant par "/public" sont accessibles sans authentification
        return Response::builder()
            .status(StatusCode::FOUND)
            .header("Location", "/login")
            .body(Body::empty())
            .unwrap();
    }

    request.extensions_mut()
        .insert(AuthState(session_tok.map(|v| (v, None, config))));

    next.run(request).await
}

impl AuthState {
    pub async fn get_user(&mut self) -> Option<&User> {
        let (session_tok, store, config) = self.0.as_mut()?;
        if store.is_none() {
            let user = users
                .inner_join(sessions)
                .filter(session_token.eq(session_tok.clone()))
                .select(User::as_select())
                .get_result(&mut config.database.pool.get().unwrap());

            match user {
                Ok(user) => *store = Some(user),
                Err(e) => {
                    eprintln!("Error retrieving user: {:?}", e);
                    return None;
                }
            }
        }
        store.as_ref()
    }
}