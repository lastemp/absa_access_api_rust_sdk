use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct TenorInputDetails {
    currency_pair: String,
}

impl TenorInputDetails {
    pub fn new(currency_pair: String) -> Result<Self, String> {
        if currency_pair.is_empty() || currency_pair.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency pair is empty"));
        }
        // currency_pair has a length of 6 characters i.e USDKES
        else if currency_pair.trim().len() == 6 {
            // currency_pair is valid
        } else {
            return Err(String::from("currency pair has invalid length"));
        }

        Ok(Self { currency_pair })
    }

    pub fn get_currency_pair(&self) -> String {
        let currency_pair = &self.currency_pair;
        currency_pair.to_string()
    }
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TenorData {
    pub long: Option<String>,
    pub short: Option<String>,
    pub value: Option<u32>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TenorResponseData {
    pub tenors: Vec<TenorData>,
}
