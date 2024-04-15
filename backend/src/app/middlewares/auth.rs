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
const USER_COOKIE_NAME: &str = "session_token";

pub(crate) async fn auth(
    config: Arc<Config>,
    mut request: Request, next: Next,
) -> axum::response::Response {
    let session_toks: Vec<_> = request.headers().get_all("Cookie").iter().filter_map(|cookie| {
            cookie.to_str().ok().and_then(|cookie| cookie.parse::<cookie::Cookie>().ok())
        })
        .filter(|cookie| cookie.name() == USER_COOKIE_NAME)
        .map(|cookie| cookie.value().to_owned())
        .collect();
    let is_logged_in = !session_toks.is_empty();

    let path = request.uri().path().to_owned();

    if is_logged_in {
        let session_tok = session_toks.first().map(|v| v.clone());
        let mut auth_state = AuthState(session_tok.map(|v| (v, None, config)));
        request.extensions_mut().insert(auth_state.clone());
        if auth_state.get_user().await.is_none() {
            return auth_state.redirect_to_login();
        } else if path == "/login" {
            return Response::builder()
            .status(StatusCode::FOUND)
            .header("Location", "/")
            .body(Body::empty())
            .unwrap();
        }
       
    } else if !is_logged_in && path != "/login" && !path.starts_with("/public") {
        return Response::builder()
        .status(StatusCode::FOUND)
            .header("Location", "/login")
            .body(Body::empty())
            .unwrap();
    }

    next.run(request).await
}

/// AuthState is a structure designed to manage authentication state.
///
/// This structure provides a secure way to store and retrieve authentication state,
/// including the authentication token, the currently authenticated user, and the authentication configuration.
/// It uses an optional tuple to manage these pieces of information, and an Arc<Mutex<HashMap<String, String>>> for the configuration,
/// ensuring secure and concurrent access to the authentication state.
///
/// # Examples
///
/// ```
#[derive(Clone)]
pub struct AuthState(Option<(String, Option<User>, Arc<Config>)>);

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
                Err(_e) => {
                    return None;
                }
            }
        }
        store.as_ref()
    }

    pub fn redirect_to_login(&self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header("Location", "/login")
            .header("Set-Cookie", "session_token=; Max-Age=0")
            .body(Body::empty())
            .unwrap()
    }
}