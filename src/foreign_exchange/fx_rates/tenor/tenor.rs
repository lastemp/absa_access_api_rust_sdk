use reqwest::StatusCode;

use crate::{
    models::foreign_exchange::fx_rates::tenor::tenor::TenorResponseData, util::util::build_headers,
};

pub async fn enquire(
    access_token: String,
    api_url: String,
) -> std::result::Result<TenorResponseData, String> {
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
                match response.json::<TenorResponseData>().await {
                    Ok(tenor_response_data) => {
                        // Handle success case

                        return Ok(tenor_response_data);
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
