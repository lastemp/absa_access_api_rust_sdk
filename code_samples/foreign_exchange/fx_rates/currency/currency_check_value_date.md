# currency_check_value_date

This API checks the validity of a value date for a specific currency pair.

## main.rs

This should contain below code:

```rust
pub mod foreign_exchange {
    pub mod fx_rates {
        pub mod currency {
            pub mod currency_check_value_date;
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

    let x = foreign_exchange::fx_rates::currency::currency_check_value_date::test_check_value_date_by_currency_pair(
            consumer_key,
            consumer_secret,
            _env,
        );
	
    x.await;
}
```

## currency_check_value_date.rs

This module contains the function test_check_value_date_by_currency_pair:

```rust
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
```
