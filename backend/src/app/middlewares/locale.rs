use crate::config::application::Config;
use axum::{
    middleware::Next,
    extract::Request,
};
use serde::Deserialize;
use axum::extract::Query;
use axum::RequestPartsExt;

#[derive(Deserialize, Debug)]
struct Params {
    locale: Option<String>,
}

pub(crate) async fn change_locale(
    mut config: Config,
    request: Request, next: Next,
) -> axum::response::Response {
    let (mut parts, body) = request.into_parts();

    let params: Query<Params> = parts.extract().await.expect("REASON");
    match &params.locale {
        Some(locale) => {
            config.change_locale(locale.to_string());
        },
        None => {},
    }

    

    // Reconstruire la requête avec les mêmes parties et corps
    let request = Request::from_parts(parts, body);
    next.run(request).await
}