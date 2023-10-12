use absa_access_api_rust_sdk::models::foreign_exchange::fx_rates::currency::currency::{
    CurrencyCheckValueDateInputDetails, CurrencyCheckValueDateResponseData,
    UnauthorizedErrorResponseData,
};
use absa_access_api_rust_sdk::AbsaAccessGateway;

pub async fn test_check_value_date_by_currency_pair(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = AbsaAccessGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(absa_access) = _result {
        let currency_pair = String::from("***");
        let value_date = String::from("***");

        let _result = CurrencyCheckValueDateInputDetails::new(currency_pair, value_date);

        if let Ok(currency_details) = _result {
            let _output = absa_access.check_value_date_by_currency_pair(currency_details);
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
