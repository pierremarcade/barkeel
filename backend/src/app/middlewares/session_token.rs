use axum::{
    http::{header,HeaderValue, HeaderMap},
    response::Response,
    middleware::Next,
    extract::Request,
};
use cookie::Cookie;
use std::time::{SystemTime, UNIX_EPOCH};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use rand::Rng;

#[derive(Clone)]
pub struct UniqueId(pub String);

pub async fn unique_id_middleware(request: Request, next: Next) -> Response {
    let headers: &HeaderMap = request.headers();
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            if cookie_str.contains("session_token=") {
                let response = next.run(request).await;
                return response;
            }
        }
    }
    let unique_id = generate_unique_id();
    let cookie = Cookie::build(("session_token", unique_id)).path("/").http_only(true);
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );
    response
}

fn generate_unique_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    let random: u32 = rand::thread_rng().gen();
    let secret_string = format!("{}-{}", now, random);
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret_string.as_bytes()).unwrap();
    let claims: BTreeMap<String, String> = BTreeMap::new();
    claims.sign_with_key(&key).unwrap()
}