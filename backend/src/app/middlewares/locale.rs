use axum::{
    http::header,
    middleware::Next,
    extract::Request,
};
use cookie::Cookie;
use serde::Deserialize;
use axum::extract::Query;
use axum::RequestPartsExt;
use crate::config::constants::{LOCALE_COOKIE_NAME, DEFAULT_LOCALE};

#[derive(Deserialize, Debug)]
struct Params {
    locale: Option<String>,
}

pub(crate) async fn change_locale(request: Request, next: Next) -> axum::response::Response {
    let cookie_header = request.headers().get(header::COOKIE);
    let mut cookie = match cookie_header {
        Some(cookie_header) => {
            let cookies: Vec<Cookie> = cookie_header.to_str().unwrap_or_default()
                .split(';').filter_map(|s| s.trim().parse::<Cookie>().ok())
                .collect();
            let locale_cookie = cookies.iter().find(|cookie| cookie.name() == LOCALE_COOKIE_NAME);
            match locale_cookie {
                Some(cookie) => cookie.clone().into_owned(),
                None => Cookie::new(LOCALE_COOKIE_NAME, DEFAULT_LOCALE),
            }
        },
        None => Cookie::new(LOCALE_COOKIE_NAME, DEFAULT_LOCALE),
    };

    let (mut parts, body) = request.into_parts();
    let params: Query<Params> = parts.extract().await.expect("REASON");
    match &params.locale {
        Some(locale) => {
            cookie.set_value(locale);
        },
        None => {},
    }
    let request = Request::from_parts(parts, body);
    next.run(request).await
}