use reqwest::StatusCode;

use crate::{
    models::foreign_exchange::fx_rates::currency::currency::{
        CurrencyCheckValueDateResponseData, UnauthorizedErrorResponseData,
    },
    util::util::build_headers,
};

pub async fn check(
    access_token: String,
    api_url: String,
) -> std::result::Result<
    (
        Option<CurrencyCheckValueDateResponseData>,
        Option<UnauthorizedErrorResponseData>,
    ),
    String,
> {
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
                match response.json::<CurrencyCheckValueDateResponseData>().await {
                    Ok(account_response_data) => {
                        // Handle success case

                        let my_output = (Some(account_response_data), None);

                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            StatusCode::UNAUTHORIZED => {
                // 401
                match response.json::<UnauthorizedErrorResponseData>().await {
                    Ok(account_response_data) => {
                        // Handle success case

                        let my_output = (None, Some(account_response_data));
                        return Ok(my_output);
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
