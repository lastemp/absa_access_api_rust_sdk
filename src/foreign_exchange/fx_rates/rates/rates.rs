use reqwest::StatusCode;

use crate::{
    models::foreign_exchange::fx_rates::rates::rates::RatesResponseData,
    util::util::build_headers_rates,
};

pub async fn enquire(
    fx_identifier: String,
    fx_identifier_type: String,
    access_token: String,
    api_url: String,
) -> std::result::Result<RatesResponseData, String> {
    let client = reqwest::Client::new();

    let res = client
        .get(api_url)
        .headers(build_headers_rates(
            fx_identifier,
            fx_identifier_type,
            access_token,
        ))
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::OK => {
                // 200
                match response.json::<RatesResponseData>().await {
                    Ok(rate_response_data) => {
                        // Handle success case

                        return Ok(rate_response_data);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
