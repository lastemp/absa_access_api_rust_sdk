use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct RateInputDetails {
    fx_identifier: String,
    fx_identifier_type: String,
    rate_token: String,
}

impl RateInputDetails {
    pub fn new(
        fx_identifier: String,
        fx_identifier_type: String,
        rate_token: String,
    ) -> Result<Self, String> {
        if fx_identifier.is_empty() || fx_identifier.replace(" ", "").trim().len() == 0 {
            return Err(String::from("fx identifier is empty"));
        }
        // fx_identifier has a max length of 200 characters
        else if fx_identifier.trim().len() > 0 && fx_identifier.trim().len() == 200 {
            // fx_identifier is valid
        } else {
            return Err(String::from("fx identifier has invalid length"));
        }

        if fx_identifier_type.is_empty() || fx_identifier_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("fx identifier type is empty"));
        }
        // fx_identifier_type has a max length of 200 characters
        else if fx_identifier_type.trim().len() > 0 && fx_identifier_type.trim().len() == 200 {
            // fx_identifier_type is valid
        } else {
            return Err(String::from("fx identifier type has invalid length"));
        }

        if rate_token.is_empty() || rate_token.replace(" ", "").trim().len() == 0 {
            return Err(String::from("rate token is empty"));
        }
        // rate_token has a max length of 200 characters
        else if rate_token.trim().len() > 0 && rate_token.trim().len() == 200 {
            // rate_token is valid
        } else {
            return Err(String::from("rate token has invalid length"));
        }

        Ok(Self {
            fx_identifier,
            fx_identifier_type,
            rate_token,
        })
    }

    pub fn get_fx_identifier(&self) -> String {
        let fx_identifier = &self.fx_identifier;
        fx_identifier.to_string()
    }

    pub fn get_fx_identifier_type(&self) -> String {
        let fx_identifier_type = &self.fx_identifier_type;
        fx_identifier_type.to_string()
    }

    pub fn get_rate_token(&self) -> String {
        let rate_token = &self.rate_token;
        rate_token.to_string()
    }
}

#[derive(Debug)]
pub struct RateLatestInputDetails {
    fx_identifier: String,
    fx_identifier_type: String,
    currency_pair: String,
    _tenor: String,
    country_code: Option<String>,
}

impl RateLatestInputDetails {
    pub fn new(
        fx_identifier: String,
        fx_identifier_type: String,
        currency_pair: String,
        _tenor: String,
        country_code: Option<String>,
    ) -> Result<Self, String> {
        if fx_identifier.is_empty() || fx_identifier.replace(" ", "").trim().len() == 0 {
            return Err(String::from("fx identifier is empty"));
        }
        // fx_identifier has a max length of 200 characters
        else if fx_identifier.trim().len() > 0 && fx_identifier.trim().len() == 200 {
            // fx_identifier is valid
        } else {
            return Err(String::from("fx identifier has invalid length"));
        }

        if fx_identifier_type.is_empty() || fx_identifier_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("fx identifier type is empty"));
        }
        // fx_identifier_type has a max length of 200 characters
        else if fx_identifier_type.trim().len() > 0 && fx_identifier_type.trim().len() == 200 {
            // fx_identifier_type is valid
        } else {
            return Err(String::from("fx identifier type has invalid length"));
        }

        if currency_pair.is_empty() || currency_pair.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency pair is empty"));
        }
        // currency_pair has a length of 6 characters i.e USDKES
        else if currency_pair.trim().len() == 6 {
            // currency_pair is valid
        } else {
            return Err(String::from("currency pair has invalid length"));
        }

        if _tenor.is_empty() || _tenor.replace(" ", "").trim().len() == 0 {
            return Err(String::from("tenor is empty"));
        }
        // _tenor has a max length of 100 characters
        // _tenor i.e Settlement date (value date) in format yyyyMMdd or tenor.
        else if _tenor.trim().len() > 0 && _tenor.trim().len() == 100 {
            // _tenor is valid
        } else {
            return Err(String::from("tenor has invalid length"));
        }

        if let Some(country_code) = &country_code {
            if country_code.is_empty() || country_code.replace(" ", "").trim().len() == 0 {
                return Err(String::from("country code is empty"));
            }
            // country_code has a length of 2 characters i.e KE
            else if country_code.trim().len() == 2 {
                // country_code is valid
            } else {
                return Err(String::from("country code has invalid length"));
            }
        }

        Ok(Self {
            fx_identifier,
            fx_identifier_type,
            currency_pair,
            _tenor,
            country_code,
        })
    }

    pub fn get_fx_identifier(&self) -> String {
        let fx_identifier = &self.fx_identifier;
        fx_identifier.to_string()
    }

    pub fn get_fx_identifier_type(&self) -> String {
        let fx_identifier_type = &self.fx_identifier_type;
        fx_identifier_type.to_string()
    }

    pub fn get_currency_pair(&self) -> String {
        let currency_pair = &self.currency_pair;
        currency_pair.to_string()
    }

    pub fn get_tenor(&self) -> String {
        let _tenor = &self._tenor;
        _tenor.to_string()
    }

    pub fn get_country_code(&self) -> &Option<String> {
        let country_code = &self.country_code;
        country_code
    }
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TierData {
    pub maxQtyInBaseCcy: Option<f32>,
    pub bid: Option<f32>,
    pub offer: Option<f32>,
    pub spotBid: Option<f32>,
    pub spotOffer: Option<f32>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct RateResponseData {
    pub token: Option<String>,
    pub timestamp: Option<String>,
    pub valueDate: Option<String>,
    pub timeToLive: Option<String>,
    pub primaryCurrency: Option<String>,
    pub secondaryCurrency: Option<String>,
    pub tenor: Option<String>,
    pub liquidityLimitInBaseCcy: Option<f32>,
    pub pointMultiplier: Option<u32>,
    pub firstCrosserCurrencyPair: Option<String>,
    pub firstCrosserBidRate: Option<f32>,
    pub firstCrosserOfferRate: Option<f32>,
    pub secondCrosserCurrencyPair: Option<String>,
    pub secondCrosserBidRate: Option<f32>,
    pub secondCrosserOfferRate: Option<f32>,
    pub tiers: Vec<TierData>,
}
