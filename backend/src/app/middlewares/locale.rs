use crate::config::application::Config;
use axum::{
    middleware::Next,
    extract::Request,
};
use std::sync::Mutex;
use serde::Deserialize;
use std::sync::Arc;
use axum::extract::Query;
use axum::RequestPartsExt;

#[derive(Deserialize, Debug)]
struct Params {
    locale: Option<String>,
}

pub(crate) async fn change_locale(
    config: Arc<Mutex<Config>>,
    request: Request, next: Next,
) -> axum::response::Response {
    let (mut parts, body) = request.into_parts();

    let params: Query<Params> = parts.extract().await.expect("REASON");
    match &params.locale {
        Some(locale) => {
            if let Ok(mut config_guard) = config.lock() {
                config_guard.change_locale(locale.to_string());
            }
        },
        None => {},
    }

    

    // Reconstruire la requête avec les mêmes parties et corps
    let request = Request::from_parts(parts, body);
    next.run(request).await
}