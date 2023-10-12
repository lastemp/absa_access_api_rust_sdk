pub mod foreign_exchange {
    pub mod fx_rates {
        pub mod currency {
            pub mod currency;
            pub mod currency_check_value_date;
            pub mod currency_cutoff_time;
        }
        pub mod rate {
            pub mod rate;
            pub mod rate_latest;
        }
        pub mod tenor {
            pub mod tenor;
        }
        pub mod rates {
            pub mod rates;
            pub mod teller_historic_rates;
            pub mod teller_rates;
        }
    }
    pub mod fx_trades {
        pub mod deal {
            pub mod deal;
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

    // currency

    let x = foreign_exchange::fx_rates::currency::currency::test_enquire_currencies(
        consumer_key,
        consumer_secret,
        _env,
    );
    /*
        let x = foreign_exchange::fx_rates::currency::currency_check_value_date::test_check_value_date_by_currency_pair(
            consumer_key,
            consumer_secret,
            _env,
        );

        let x = foreign_exchange::fx_rates::currency::currency_cutoff_time::test_enquire_currencies_cutoff_time(
            consumer_key,
            consumer_secret,
            _env,
        );
    */

    // rate
    /*
        let x = foreign_exchange::fx_rates::rate::rate::test_enquire_rate(
            consumer_key,
            consumer_secret,
            _env,
        );

        let x = foreign_exchange::fx_rates::rate::rate_latest::test_enquire_rate_latest(
            consumer_key,
            consumer_secret,
            _env,
        );
    */

    // tenor
    /*
    let x = foreign_exchange::fx_rates::tenor::tenor::test_enquire_tenors(
        consumer_key,
        consumer_secret,
        _env,
    );
    */

    // rates
    /*
        let x = foreign_exchange::fx_rates::rates::rates::test_enquire_rates_latest(
            consumer_key,
            consumer_secret,
            _env,
        );

        let x = foreign_exchange::fx_rates::rates::teller_rates::test_enquire_teller_rates(
            consumer_key,
            consumer_secret,
            _env,
        );

        let x = foreign_exchange::fx_rates::rates::teller_historic_rates::test_enquire_teller_historic_rates(
            consumer_key,
            consumer_secret,
            _env,
        );
    */

    // deal
    /*
    let x = foreign_exchange::fx_trades::deal::deal::test_book_deal(
        consumer_key,
        consumer_secret,
        _env,
    );
    */
    x.await;
}
