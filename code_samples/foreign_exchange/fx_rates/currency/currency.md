# currency

This API retrieves a list of currencies.

## main.rs

This should contain below code:

```rust
pub mod foreign_exchange {
    pub mod fx_rates {
        pub mod currency {
            pub mod currency;
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

    let x = foreign_exchange::fx_rates::currency::currency::test_enquire_currencies(
        consumer_key,
        consumer_secret,
        _env,
    );
	
    x.await;
}
```

## currency.rs

This module contains the function test_enquire_currencies:

```rust
use absa_access_api_rust_sdk::models::foreign_exchange::fx_rates::currency::currency::{
    CurrencyResponseData, UnauthorizedErrorResponseData,
};
use absa_access_api_rust_sdk::AbsaAccessGateway;

pub async fn test_enquire_currencies(consumer_key: String, consumer_secret: String, _env: String) {
    let _result = AbsaAccessGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(absa_access) = _result {
        let _output = absa_access.enquire_currencies();
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
}
```
