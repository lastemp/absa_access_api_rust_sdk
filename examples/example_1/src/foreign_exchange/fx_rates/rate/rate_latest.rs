use absa_access_api_rust_sdk::models::foreign_exchange::fx_rates::rate::rate::{
    RateLatestInputDetails, RateResponseData,
};
use absa_access_api_rust_sdk::AbsaAccessGateway;

pub async fn test_enquire_rate_latest(consumer_key: String, consumer_secret: String, _env: String) {
    let _result = AbsaAccessGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(absa_access) = _result {
        let fx_identifier = String::from("***");
        let fx_identifier_type = String::from("***");
        let currency_pair = String::from("***");
        let _tenor = String::from("***");
        let country_code: Option<String> = Some(String::from("***"));

        let _result = RateLatestInputDetails::new(
            fx_identifier,
            fx_identifier_type,
            currency_pair,
            _tenor,
            country_code,
        );

        if let Ok(rate_details) = _result {
            let _output = absa_access.enquire_rate_latest(rate_details);
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
