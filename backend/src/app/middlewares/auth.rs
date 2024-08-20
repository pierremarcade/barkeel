use crate::config::application::Config;
use crate::app::models::user::User;
use axum::{
    middleware::Next,
    extract::Request, 
    http::StatusCode,  
    body::Body,
    response::Response
};
use crate::db::schema::users::dsl::*;
use diesel::prelude::*;
use cookie::Cookie;
const USER_COOKIE_NAME: &str = "session_token";

pub(crate) async fn auth(
    config: Config,
    mut request: Request, next: Next,
) -> axum::response::Response {
    let cookie_header = request.headers().get("Cookie");
    match cookie_header {
        Some(cookie_header) => {
            let cookies: Vec<Cookie> = cookie_header
                .to_str()
                .unwrap_or_default()
                .split(';')
                .filter_map(|s| s.trim().parse::<Cookie>().ok())
                .collect();
            let session_cookie = cookies.iter().find(|cookie| cookie.name() == USER_COOKIE_NAME);
            let path = request.uri().path().to_owned();
            match session_cookie {
                Some(cookie) => {
                    let mut auth_state = AuthState(Some((cookie.value().to_string(), None, config)));
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
                },
                None => {
                    if path != "/login" && !path.starts_with("/public") {
                        return Response::builder()
                        .status(StatusCode::FOUND)
                            .header("Location", "/login")
                            .body(Body::empty())
                            .unwrap();
                    }
                },
            }
        },
        None => {
            return Response::builder()
            .status(StatusCode::FOUND)
                .header("Location", "/login")
                .body(Body::empty())
                .unwrap();
        }
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
///
/// ```
#[derive(Clone)]
pub struct AuthState(Option<(String, Option<User>, Config)>);

impl AuthState {
    pub async fn get_user(&mut self) -> Option<&User> {
        let (session_tok, store, config) = self.0.as_mut()?;
        if store.is_none() {
            let user = users
                .filter(session_token.eq(session_tok.clone()))
                .first::<User>(&mut config.database.pool.get().unwrap());

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