use absa_access_api_rust_sdk::models::foreign_exchange::fx_trades::deal::deal::{
    DealInputDetails, DealResponseData, ErrorResponseData,
};
use absa_access_api_rust_sdk::AbsaAccessGateway;

pub async fn test_book_deal(consumer_key: String, consumer_secret: String, _env: String) {
    let _result = AbsaAccessGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(absa_access) = _result {
        let fx_identifier = String::from("***");
        let fx_identifier_type = String::from("***");
        let rate_token = String::from("***");

        let _amount: f32 = 1000.0;
        let buy_sell = String::from("***");
        let _currency = String::from("***");
        let client_id: u32 = 245;
        let client_reference = String::from("***");
        let _token: String = String::from("***");
        let source_id = String::from("***");
        let purchase_nostro = String::from("***");
        let sales_nostro = String::from("***");
        let option_type = String::from("***");
        let option_start_date = String::from("***");
        let _jurisdiction = String::from("***");
        let rfq_id = String::from("***");

        let _result = DealInputDetails::new(
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

        if let Ok(rate_details) = _result {
            let _output = absa_access.book_deal(rate_details);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
            } else if let Err(e) = _result {
                println!("{:?}", e);
            } else {
                println!("Unexpected error occured during processing");
            }
        } else if let Err(e) = _result {
            println!("{:?}", e);
        } else {
            println!("Unexpected error occured during processing");
        }
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}
