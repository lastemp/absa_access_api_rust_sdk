use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

pub fn build_headers(access_token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", access_token.parse().unwrap());

    headers
}

pub fn build_headers_rate(
    fx_identifier: String,
    fx_identifier_type: String,
    access_token: String,
) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", access_token.parse().unwrap());
    headers.insert("Fx-Identifier", fx_identifier.parse().unwrap());
    headers.insert("Fx-Identifier-Type", fx_identifier_type.parse().unwrap());

    headers
}

pub fn build_headers_rates(
    fx_identifier: String,
    fx_identifier_type: String,
    access_token: String,
) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", access_token.parse().unwrap());
    headers.insert("Fx-Identifier", fx_identifier.parse().unwrap());
    headers.insert("Fx-Identifier-Type", fx_identifier_type.parse().unwrap());

    headers
}

pub fn build_headers_generate_auth_token(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", api_key.parse().unwrap());

    headers
}
