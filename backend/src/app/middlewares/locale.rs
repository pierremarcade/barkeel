use crate::config::application::Config;
use axum::{
    middleware::Next,
    extract::Request, 
    http::StatusCode,  
    body::Body,
    response::Response
};
use std::sync::Mutex;
use serde::Deserialize;
use std::sync::Arc;
use axum::extract::Query;
use axum::RequestPartsExt;
use unic_langid::{LanguageIdentifier, langid};

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
            let mut config_guard = config.lock().unwrap();
            config_guard.change_locale(langid!("fr"));
        },
        None => {},
    }

    

    // Reconstruire la requête avec les mêmes parties et corps
    let request = Request::from_parts(parts, body);
    next.run(request).await
}