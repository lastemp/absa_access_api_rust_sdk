pub mod models {
    pub mod foreign_exchange {
        pub mod fx_rates {
            pub mod currency {
                pub mod currency;
            }
            pub mod rate {
                pub mod rate;
            }
            pub mod tenor {
                pub mod tenor;
            }
            pub mod rates {
                pub mod rates;
            }
        }
    }

    pub mod authorization {
        pub mod auth_token;
    }
}
mod util {
    pub mod util;
}
mod authorization {
    pub mod generate_auth_token;
}
pub mod foreign_exchange {
    pub mod fx_rates {
        pub mod currency {
            pub mod currency;
            pub mod currency_check_value_date;
            pub mod currency_cutoff_time;
        }
        pub mod rate {
            pub mod rate;
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
}
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

use models::foreign_exchange::fx_rates::{
    currency::currency::{
        CurrencyCheckValueDateInputDetails, CurrencyCheckValueDateResponseData,
        CurrencyCutoffTimeInputDetails, CurrencyCutoffTimeResponseData, CurrencyResponseData,
        UnauthorizedErrorResponseData,
    },
    rate::rate::{RateInputDetails, RateLatestInputDetails, RateResponseData},
    rates::rates::{
        RatesLatestInputDetails, RatesResponseData, TellerHistoricRateInputDetails,
        TellerHistoricRateResponseData, TellerRateResponseData,
    },
    tenor::tenor::{TenorInputDetails, TenorResponseData},
};

const AUTHORISATION_BEARER: &str = "Bearer";
const GRANT_TYPE: &str = "client_credentials";

const AUTH_TOKEN_URL_SANDBOX: &str =
    "https://mtls-auth-za.ppe.absaaccess.cib.digital/connect/mtls/token";
const AUTH_TOKEN_URL_PROD: &str =
    "https://mtls-auth-za.ppe.absaaccess.cib.digital/connect/mtls/token";

const CURRENCY_URL_SANDBOX: &str =
    "https://gateway.bifrost.cib-absaaccess.uat.caas.absa.co.za/fxratesapi/1.0/api/currencies";
const CURRENCY_URL_PROD: &str =
    "https://gateway.bifrost.cib-absaaccess.prod.caas.absa.co.za/fxratesapi/1.0/api/currencies";

const RATE_URL_SANDBOX: &str =
    "https://gateway.bifrost.cib-absaaccess.uat.caas.absa.co.za/fxratesapi/1.0/rate/";
const RATE_URL_PROD: &str =
    "https://gateway.bifrost.cib-absaaccess.prod.caas.absa.co.za/fxratesapi/1.0/rate/";

const TENOR_URL_SANDBOX: &str =
    "https://gateway.bifrost.cib-absaaccess.uat.caas.absa.co.za/fxratesapi/1.0/tenors";
const TENOR_URL_PROD: &str =
    "https://gateway.bifrost.cib-absaaccess.prod.caas.absa.co.za/fxratesapi/1.0/tenors";

const RATES_URL_SANDBOX: &str =
    "https://gateway.bifrost.cib-absaaccess.uat.caas.absa.co.za/fxratesapi/1.0/rates/";
const RATES_URL_PROD: &str =
    "https://gateway.bifrost.cib-absaaccess.prod.caas.absa.co.za/fxratesapi/1.0/rates/";

#[derive(Debug)]
pub struct AbsaAccessGateway {
    grant_type: String,
    consumer_key: String,
    consumer_secret: String,
    auth_token_url: String,
    currency_url: String,
    rate_url: String,
    tenor_url: String,
    rates_url: String,
}

impl AbsaAccessGateway {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        _env: String,
    ) -> Result<Self, String> {
        if consumer_key.is_empty() || consumer_key.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer key is empty"));
        }

        if consumer_secret.is_empty() || consumer_secret.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer secret is empty"));
        }

        if _env.is_empty() || _env.replace(" ", "").trim().len() == 0 {
            return Err(String::from("_env is empty"));
        }

        if _env.eq_ignore_ascii_case(&String::from("sandbox"))
            || _env.eq_ignore_ascii_case(&String::from("prod"))
        {
            // valid _env
        } else {
            return Err(String::from("invalid env"));
        }

        let grant_type = GRANT_TYPE.to_string();

        let auth_token_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            AUTH_TOKEN_URL_PROD.to_string()
        } else {
            AUTH_TOKEN_URL_SANDBOX.to_string()
        };

        let currency_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            CURRENCY_URL_PROD.to_string()
        } else {
            CURRENCY_URL_SANDBOX.to_string()
        };

        let rate_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            RATE_URL_PROD.to_string()
        } else {
            RATE_URL_SANDBOX.to_string()
        };

        let tenor_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            TENOR_URL_PROD.to_string()
        } else {
            TENOR_URL_SANDBOX.to_string()
        };

        let rates_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            RATES_URL_PROD.to_string()
        } else {
            RATES_URL_SANDBOX.to_string()
        };

        Ok(Self {
            grant_type,
            consumer_key,
            consumer_secret,
            auth_token_url,
            currency_url,
            rate_url,
            tenor_url,
            rates_url,
        })
    }

    fn get_api_key(&self) -> String {
        let consumer_key = &self.consumer_key;
        let consumer_secret = &self.consumer_secret;
        let mut password: String = consumer_key.to_string();
        let k = ":"; // Separator
        password.push_str(k);
        password.push_str(&consumer_secret);
        let encodedpassword = general_purpose::STANDARD.encode(password);

        let mut api_key = String::from("Basic");
        let k = " "; // Separator
        api_key.push_str(k);
        api_key.push_str(&encodedpassword);

        api_key
    }

    fn parse_auth_token(&self, access_token_result: String) -> String {
        let access_token: String = if !access_token_result.is_empty()
            && access_token_result.replace(" ", "").trim().len() > 0
        {
            let mut access_token = AUTHORISATION_BEARER.to_string();
            let k = " "; // Separator
            access_token.push_str(k);
            access_token.push_str(&access_token_result);

            access_token
        } else {
            String::from("")
        };

        access_token
    }

    async fn get_auth_token(&self) -> std::result::Result<String, String> {
        let grant_type = &self.grant_type;
        let api_key = &self.get_api_key();
        let api_url = &self.auth_token_url;

        let _result = authorization::generate_auth_token::get_auth_token(
            grant_type.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        )
        .await;

        _result
    }

    pub async fn enquire_currencies(
        &self,
    ) -> std::result::Result<
        (
            Option<CurrencyResponseData>,
            Option<UnauthorizedErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.currency_url;

                let _result = foreign_exchange::fx_rates::currency::currency::enquire(
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_currencies_cutoff_time(
        &self,
        currency_details: CurrencyCutoffTimeInputDetails,
    ) -> std::result::Result<
        (
            Option<CurrencyCutoffTimeResponseData>,
            Option<UnauthorizedErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let currency_code: String = currency_details.get_currency_code();
                let country_code: String = currency_details.get_country_code();
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.currency_url;
                let mut api_url = api_url.to_string();
                let _a = "/";
                let _b = "cutofftimes?countryCode=";

                api_url.push_str(&_a);

                api_url.push_str(&currency_code);
                api_url.push_str(&_a);
                api_url.push_str(&_b);
                api_url.push_str(&country_code);

                let _result = foreign_exchange::fx_rates::currency::currency_cutoff_time::enquire(
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn check_value_date_by_currency_pair(
        &self,
        currency_details: CurrencyCheckValueDateInputDetails,
    ) -> std::result::Result<
        (
            Option<CurrencyCheckValueDateResponseData>,
            Option<UnauthorizedErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let currency_pair: String = currency_details.get_currency_pair();
                let value_date: String = currency_details.get_value_date();
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.currency_url;
                let mut api_url = api_url.to_string();
                let _a = "/checkvaluedate?currencyPair=";
                let _b = "&valueDate=2";

                api_url.push_str(&_a);
                api_url.push_str(&currency_pair);
                api_url.push_str(&_b);
                api_url.push_str(&value_date);

                let _result =
                    foreign_exchange::fx_rates::currency::currency_check_value_date::check(
                        access_token,
                        api_url.to_string(),
                    )
                    .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_rate(
        &self,
        rate_details: RateInputDetails,
    ) -> std::result::Result<RateResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let fx_identifier: String = rate_details.get_fx_identifier();
                let fx_identifier_type: String = rate_details.get_fx_identifier_type();
                let rate_token: String = rate_details.get_rate_token();
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.rate_url;
                let mut api_url = api_url.to_string();

                api_url.push_str(&rate_token);

                let _result = foreign_exchange::fx_rates::rate::rate::enquire(
                    fx_identifier,
                    fx_identifier_type,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_rate_latest(
        &self,
        rate_details: RateLatestInputDetails,
    ) -> std::result::Result<RateResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let fx_identifier: String = rate_details.get_fx_identifier();
                let fx_identifier_type: String = rate_details.get_fx_identifier_type();
                let currency_pair: String = rate_details.get_currency_pair();
                let _tenor: String = rate_details.get_tenor();
                let country_code = rate_details.get_country_code(); // optional value
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.rate_url;
                let mut api_url = api_url.to_string();
                let _a = "latest?currencyPair=";
                let _b = "&tenor=";

                api_url.push_str(&_a);
                api_url.push_str(&currency_pair);
                api_url.push_str(&_b);
                api_url.push_str(&_tenor);

                if let Some(country_code) = &country_code {
                    if country_code.trim().len() == 2 {
                        let _c = "&countryCode=";
                        api_url.push_str(&_c);
                        api_url.push_str(&country_code);
                    }
                }

                let _result = foreign_exchange::fx_rates::rate::rate::enquire(
                    fx_identifier,
                    fx_identifier_type,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_tenors(
        &self,
        tenor_details: TenorInputDetails,
    ) -> std::result::Result<TenorResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let currency_pair: String = tenor_details.get_currency_pair();
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.tenor_url;
                let mut api_url = api_url.to_string();
                let _a = "?currencyPair=";

                api_url.push_str(&_a);
                api_url.push_str(&currency_pair);

                let _result = foreign_exchange::fx_rates::tenor::tenor::enquire(
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_rates_latest(
        &self,
        rate_details: RatesLatestInputDetails,
    ) -> std::result::Result<RatesResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let fx_identifier: String = rate_details.get_fx_identifier();
                let fx_identifier_type: String = rate_details.get_fx_identifier_type();
                let currency_pair: String = rate_details.get_currency_pair();
                let _tenor: String = rate_details.get_tenor();
                let country_code = rate_details.get_country_code(); // optional value
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.rates_url;
                let mut api_url = api_url.to_string();
                let _a = "latest?currencyPair=";
                let _b = "&tenor=";

                api_url.push_str(&_a);
                api_url.push_str(&currency_pair);
                api_url.push_str(&_b);
                api_url.push_str(&_tenor);

                if let Some(country_code) = &country_code {
                    if country_code.trim().len() == 2 {
                        let _c = "&countryCode=";
                        api_url.push_str(&_c);
                        api_url.push_str(&country_code);
                    }
                }

                let _result = foreign_exchange::fx_rates::rates::rates::enquire(
                    fx_identifier,
                    fx_identifier_type,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_teller_rates(
        &self,
    ) -> std::result::Result<TellerRateResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case

                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.rates_url;
                let mut api_url = api_url.to_string();
                let _a = "teller/latest";

                api_url.push_str(&_a);

                let _result = foreign_exchange::fx_rates::rates::teller_rates::enquire(
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_teller_historic_rates(
        &self,
        rate_details: TellerHistoricRateInputDetails,
    ) -> std::result::Result<TellerHistoricRateResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let from_date: String = rate_details.get_from_date();
                let to_date: String = rate_details.get_to_date();
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.rates_url;
                let mut api_url = api_url.to_string();
                let _a = "teller/historic?fromDate=";
                let _b = "&toDate=";

                api_url.push_str(&_a);
                api_url.push_str(&from_date);
                api_url.push_str(&_b);
                api_url.push_str(&to_date);

                let _result = foreign_exchange::fx_rates::rates::teller_historic_rates::enquire(
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absa_access_gateway() {
        let consumer_key = String::from("***");
        let consumer_secret = String::from("***");
        let _env = String::from("sandbox");

        let _result = AbsaAccessGateway::new(consumer_key, consumer_secret, _env);
        assert_eq!(_result.is_ok(), true);
    }
}
