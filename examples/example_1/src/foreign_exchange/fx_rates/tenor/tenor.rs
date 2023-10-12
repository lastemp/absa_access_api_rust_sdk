use absa_access_api_rust_sdk::models::foreign_exchange::fx_rates::tenor::tenor::{
    TenorInputDetails, TenorResponseData,
};
use absa_access_api_rust_sdk::AbsaAccessGateway;

pub async fn test_enquire_tenors(consumer_key: String, consumer_secret: String, _env: String) {
    let _result = AbsaAccessGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(absa_access) = _result {
        let currency_pair = String::from("***");

        let _result = TenorInputDetails::new(currency_pair);

        if let Ok(tenor_details) = _result {
            let _output = absa_access.enquire_tenors(tenor_details);
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
