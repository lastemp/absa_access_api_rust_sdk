use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::models::foreign_exchange::fx_trades::deal::deal::BookDealData;

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

pub fn build_deal_data(
    _amount: f32,
    buy_sell: String,
    _currency: String,
    client_id: u32,
    client_reference: String,
    _token: String,
    source_id: String,
    purchase_nostro: String,
    sales_nostro: String,
    option_type: String,
    option_start_date: String,
    _jurisdiction: String,
    rfq_id: String,
) -> BookDealData {
    BookDealData {
        amount: _amount,
        buySell: buy_sell,
        currency: _currency,
        clientId: client_id,
        clientReference: client_reference,
        token: _token,
        sourceId: source_id,
        purchaseNostro: purchase_nostro,
        salesNostro: sales_nostro,
        optionType: option_type,
        optionStartDate: option_start_date,
        jurisdiction: _jurisdiction,
        rfqId: rfq_id,
    }
}
