use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct CurrencyCutoffTimeInputDetails {
    currency_code: String,
    country_code: String,
}

impl CurrencyCutoffTimeInputDetails {
    pub fn new(currency_code: String, country_code: String) -> Result<Self, String> {
        if currency_code.is_empty() || currency_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency code is empty"));
        }
        // currency_code has a length of 3 characters i.e KES
        else if currency_code.trim().len() == 3 {
            // currency_code is valid
        } else {
            return Err(String::from("currency code has invalid length"));
        }

        if country_code.is_empty() || country_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("country code is empty"));
        }
        // country_code has a length of 2 characters i.e KE
        else if country_code.trim().len() == 2 {
            // country_code is valid
        } else {
            return Err(String::from("country code has invalid length"));
        }

        Ok(Self {
            currency_code,
            country_code,
        })
    }

    pub fn get_currency_code(&self) -> String {
        let currency_code = &self.currency_code;
        currency_code.to_string()
    }

    pub fn get_country_code(&self) -> String {
        let country_code = &self.country_code;
        country_code.to_string()
    }
}

#[derive(Debug)]
pub struct CurrencyCheckValueDateInputDetails {
    currency_pair: String,
    value_date: String,
}

impl CurrencyCheckValueDateInputDetails {
    pub fn new(currency_pair: String, value_date: String) -> Result<Self, String> {
        if currency_pair.is_empty() || currency_pair.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency pair is empty"));
        }
        // currency_pair has a length of 6 characters i.e USDKES
        else if currency_pair.trim().len() == 6 {
            // currency_pair is valid
        } else {
            return Err(String::from("currency pair has invalid length"));
        }

        if value_date.is_empty() || value_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("value date is empty"));
        }
        // value_date has a length of 8 characters i.e format yyyyMMdd
        else if value_date.trim().len() == 8 {
            // value_date is valid
        } else {
            return Err(String::from("value date has invalid length"));
        }

        Ok(Self {
            currency_pair,
            value_date,
        })
    }

    pub fn get_currency_pair(&self) -> String {
        let currency_pair = &self.currency_pair;
        currency_pair.to_string()
    }

    pub fn get_value_date(&self) -> String {
        let value_date = &self.value_date;
        value_date.to_string()
    }
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CurrencyData {
    pub id: Option<u32>,
    pub countryCode: Option<String>,
    pub currencyCode: Option<String>,
    pub currencyRanking: Option<u32>,
    pub countryName: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CurrencyResponseData {
    pub currency: Vec<CurrencyData>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CurrencyCutoffTimeResponseData {
    pub countryCode: Option<String>,
    pub currencyCode: Option<String>,
    pub direction: Option<String>,
    pub trading: Option<String>,
    pub nstp: Option<String>,
    pub stp: Option<String>,
    pub tenor: Option<String>,
    pub nextTenor: Option<String>,
    pub isActive: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CurrencyCheckValueDateResponseData {
    pub isValid: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UnauthorizedErrorResponseData {
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub status: Option<u32>,
    pub detail: Option<String>,
    pub instance: Option<String>,
    pub additionalProp1: Option<String>,
    pub additionalProp2: Option<String>,
    pub additionalProp3: Option<String>,
}
