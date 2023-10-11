use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct RatesLatestInputDetails {
    fx_identifier: String,
    fx_identifier_type: String,
    currency_pair: String,
    _tenor: String,
    country_code: Option<String>,
}

impl RatesLatestInputDetails {
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
        else if fx_identifier.trim().len() > 0 && fx_identifier.trim().len() <= 200 {
            // fx_identifier is valid
        } else {
            return Err(String::from("fx identifier has invalid length"));
        }

        if fx_identifier_type.is_empty() || fx_identifier_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("fx identifier type is empty"));
        }
        // fx_identifier_type has a max length of 200 characters
        else if fx_identifier_type.trim().len() > 0 && fx_identifier_type.trim().len() <= 200 {
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
        else if _tenor.trim().len() > 0 && _tenor.trim().len() <= 100 {
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

#[derive(Debug)]
pub struct TellerHistoricRateInputDetails {
    from_date: String,
    to_date: String,
}

impl TellerHistoricRateInputDetails {
    pub fn new(from_date: String, to_date: String) -> Result<Self, String> {
        if from_date.is_empty() || from_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("from date is empty"));
        }
        // from_date has a length of 10 characters i.e format yyyy-MM-dd
        else if from_date.trim().len() == 10 {
            // from_date is valid
        } else {
            return Err(String::from("from date has invalid length"));
        }

        if to_date.is_empty() || to_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("to date is empty"));
        }
        // to_date has a length of 10 characters i.e format yyyy-MM-dd
        else if to_date.trim().len() == 10 {
            // to_date is valid
        } else {
            return Err(String::from("to date has invalid length"));
        }

        Ok(Self { from_date, to_date })
    }

    pub fn get_from_date(&self) -> String {
        let from_date = &self.from_date;
        from_date.to_string()
    }

    pub fn get_to_date(&self) -> String {
        let to_date = &self.to_date;
        to_date.to_string()
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
pub struct RateData {
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

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct RatesResponseData {
    pub rates: Vec<RateData>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TellerRateData {
    pub currencyCode: Option<String>,
    pub currencyName: Option<String>,
    pub isMultiply: Option<bool>,
    pub buyTransfers: Option<f32>,
    pub buyCheques: Option<f32>,
    pub buyNotesTCheques: Option<f32>,
    pub sellTChequesTransfers: Option<f32>,
    pub sellNotes: Option<f32>,
    pub date: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TellerRateResponseData {
    pub teller_rates: Vec<TellerRateData>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TellerHistoricRateData {
    pub date: Option<String>,
    pub currencyCode: Option<String>,
    pub currencyName: Option<String>,
    pub buyRate: Option<f32>,
    pub sellRate: Option<f32>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TellerHistoricRateResponseData {
    pub teller_historic_rates: Vec<TellerHistoricRateData>,
}
