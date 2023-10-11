use reqwest::StatusCode;

use crate::{
    models::foreign_exchange::fx_rates::rates::rates::TellerRateResponseData,
    util::util::build_headers,
};

pub async fn enquire(
    access_token: String,
    api_url: String,
) -> std::result::Result<TellerRateResponseData, String> {
    let client = reqwest::Client::new();

    let res = client
        .get(api_url)
        .headers(build_headers(access_token))
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::OK => {
                // 200
                match response.json::<TellerRateResponseData>().await {
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
