use absa_access_api_rust_sdk::models::foreign_exchange::fx_rates::currency::currency::{
    CurrencyCutoffTimeInputDetails, CurrencyCutoffTimeResponseData, UnauthorizedErrorResponseData,
};
use absa_access_api_rust_sdk::AbsaAccessGateway;

pub async fn test_enquire_currencies_cutoff_time(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = AbsaAccessGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(absa_access) = _result {
        let currency_code = String::from("***");
        let country_code = String::from("***");

        let _result = CurrencyCutoffTimeInputDetails::new(currency_code, country_code);

        if let Ok(currency_details) = _result {
            let _output = absa_access.enquire_currencies_cutoff_time(currency_details);
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
