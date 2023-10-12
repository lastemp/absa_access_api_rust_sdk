# currency_cutoff_time

This API gets the cutoff times for a specific currency and country.

## main.rs

This should contain below code:

```rust
pub mod foreign_exchange {
    pub mod fx_rates {
        pub mod currency {
            pub mod currency_cutoff_time;
        }
    }
}

// SANDBOX
const CONSUMER_KEY_SANDBOX: &str = "***";
const CONSUMER_SECRET_SANDBOX: &str = "***";

const ENVIRONMENT: &str = "sandbox";

#[tokio::main]
async fn main() {
    let consumer_key = CONSUMER_KEY_SANDBOX.to_string();
    let consumer_secret = CONSUMER_SECRET_SANDBOX.to_string();
    let _env = ENVIRONMENT.to_string();

    let x = foreign_exchange::fx_rates::currency::currency_cutoff_time::test_enquire_currencies_cutoff_time(
            consumer_key,
            consumer_secret,
            _env,
        );
	
    x.await;
}
```

## currency_cutoff_time.rs

This module contains the function test_enquire_currencies_cutoff_time:

```rust
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
```
