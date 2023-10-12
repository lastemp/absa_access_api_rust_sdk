use reqwest::StatusCode;

use crate::{
    models::foreign_exchange::fx_trades::deal::deal::{
        DealInputDetails, DealResponseData, ErrorResponseData,
    },
    util::util::{build_deal_data, build_headers},
};

pub async fn book(
    deal_details: DealInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<(Option<DealResponseData>, Option<ErrorResponseData>), String> {
    let _amount = deal_details.get_amount();
    let buy_sell = deal_details.get_buy_sell();
    let _currency = deal_details.get_currency();
    let client_id = deal_details.get_client_id();
    let client_reference = deal_details.get_client_reference();
    let _token = deal_details.get_token();
    let source_id = deal_details.get_source_id();
    let purchase_nostro = deal_details.get_purchase_nostro();
    let sales_nostro = deal_details.get_sales_nostro();
    let option_type = deal_details.get_option_type();
    let option_start_date = deal_details.get_option_start_date();
    let _jurisdiction = deal_details.get_jurisdiction();
    let rfq_id = deal_details.get_rfq_id();

    // Lets build the request params as a struct
    let transfer_data = build_deal_data(
        _amount,
        buy_sell,
        _currency,
        client_id,
        client_reference,
        _token,
        source_id,
        purchase_nostro,
        sales_nostro,
        option_type,
        option_start_date,
        _jurisdiction,
        rfq_id,
    );

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers(access_token))
        .json(&transfer_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            // 201-CREATED
            StatusCode::CREATED => {
                match response.json::<DealResponseData>().await {
                    Ok(deal_response_data) => {
                        // Handle success case

                        let my_output = (Some(deal_response_data), None);

                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            // , 400-BAD_REQUEST, 401-UNAUTHORIZED, 404- NOT_FOUND
            StatusCode::BAD_REQUEST | StatusCode::UNAUTHORIZED | StatusCode::NOT_FOUND => {
                match response.json::<ErrorResponseData>().await {
                    Ok(deal_error_response_data) => {
                        // Handle success case

                        let my_output = (None, Some(deal_error_response_data));
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
