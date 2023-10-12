use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DealInputDetails {
    _amount: f32,
    buy_sell: String,
    _currency: String,
    client_id: u32,
    client_reference: String,
    _token: String,
    source_id: String,
    purchase_nostro: String,
    sales_nostro: String,
    option_type: String,
    option_start_date: String,
    _jurisdiction: String,
    rfq_id: String,
}

impl DealInputDetails {
    pub fn new(
        _amount: f32,
        buy_sell: String,
        _currency: String,
        client_id: u32,
        client_reference: String,
        _token: String,
        source_id: String,
        purchase_nostro: String,
        sales_nostro: String,
        option_type: String,
        option_start_date: String,
        _jurisdiction: String,
        rfq_id: String,
    ) -> Result<Self, String> {
        if _amount > 0.0 { // _amount is valid
        } else {
            return Err(String::from("amount has invalid value"));
        }

        if buy_sell.is_empty() || buy_sell.replace(" ", "").trim().len() == 0 {
            return Err(String::from("buy sell is empty"));
        }
        // buy_sell has possible values buy or sell
        else if buy_sell.eq_ignore_ascii_case(&String::from("buy"))
            || buy_sell.eq_ignore_ascii_case(&String::from("sell"))
        {
            // buy_sell is valid
        } else {
            return Err(String::from("buy sell has invalid value"));
        }

        if _currency.is_empty() || _currency.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency is empty"));
        }
        // _currency has a length of 3 characters i.e KES
        else if _currency.trim().len() == 3 {
            // _currency is valid
        } else {
            return Err(String::from("currency has invalid length"));
        }

        if client_id > 0 { // client_id is valid
        } else {
            return Err(String::from("client id has invalid value"));
        }

        if client_reference.is_empty() || client_reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("client reference is empty"));
        }
        // client_reference has a max length of 100 characters
        else if client_reference.trim().len() >= 0 && client_reference.trim().len() <= 100 {
            // client_reference is valid
        } else {
            return Err(String::from("client reference has invalid length"));
        }

        if _token.is_empty() || _token.replace(" ", "").trim().len() == 0 {
            return Err(String::from("token is empty"));
        }
        // _token has a max length of 100 characters
        else if _token.trim().len() >= 0 && _token.trim().len() <= 100 {
            // _token is valid
        } else {
            return Err(String::from("token has invalid length"));
        }

        if source_id.is_empty() || source_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("source id is empty"));
        }
        // source_id has a max length of 100 characters
        else if source_id.trim().len() >= 0 && source_id.trim().len() <= 100 {
            // source_id is valid
        } else {
            return Err(String::from("source id has invalid length"));
        }

        if purchase_nostro.is_empty() || purchase_nostro.replace(" ", "").trim().len() == 0 {
            return Err(String::from("purchase nostro is empty"));
        }
        // purchase_nostro has a max length of 100 characters
        else if purchase_nostro.trim().len() >= 0 && purchase_nostro.trim().len() <= 100 {
            // purchase_nostro is valid
        } else {
            return Err(String::from("purchase nostro has invalid length"));
        }

        if sales_nostro.is_empty() || sales_nostro.replace(" ", "").trim().len() == 0 {
            return Err(String::from("sales nostro is empty"));
        }
        // sales_nostro has a max length of 100 characters
        else if sales_nostro.trim().len() >= 0 && sales_nostro.trim().len() <= 100 {
            // sales_nostro is valid
        } else {
            return Err(String::from("sales nostro has invalid length"));
        }

        if option_type.is_empty() || option_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("option type is empty"));
        }
        // option_type has possible values buy or sell
        else if option_type.eq_ignore_ascii_case(&String::from("FIXED"))
            || option_type.eq_ignore_ascii_case(&String::from("PARTIAL"))
            || option_type.eq_ignore_ascii_case(&String::from("FULL"))
        {
            // option_type is valid
        } else {
            return Err(String::from("option type has invalid length"));
        }

        if option_start_date.is_empty() || option_start_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("option start date is empty"));
        }
        // option_start_date has a length of 24 characters i.e 2023-10-12T07:48:54.149Z
        else if option_start_date.trim().len() == 24 {
            // option_start_date is valid
        } else {
            return Err(String::from("option start date has invalid length"));
        }

        if _jurisdiction.is_empty() || _jurisdiction.replace(" ", "").trim().len() == 0 {
            return Err(String::from("jurisdiction is empty"));
        }
        // _jurisdiction has a length of 2 characters i.e KE
        else if _jurisdiction.trim().len() == 2 {
            // _jurisdiction is valid
        } else {
            return Err(String::from("jurisdiction has invalid length"));
        }

        if rfq_id.is_empty() || rfq_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("rfq id is empty"));
        }
        // rfq_id has a max length of 100 characters
        else if rfq_id.trim().len() >= 0 && rfq_id.trim().len() <= 100 {
            // rfq_id is valid
        } else {
            return Err(String::from("rfq id has invalid length"));
        }

        Ok(Self {
            _amount,
            buy_sell,
            _currency,
            client_id,
            client_reference,
            _token,
            source_id,
            purchase_nostro,
            sales_nostro,
            option_type,
            option_start_date,
            _jurisdiction,
            rfq_id,
        })
    }

    pub fn get_amount(&self) -> f32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_buy_sell(&self) -> String {
        let buy_sell = &self.buy_sell;
        buy_sell.to_string()
    }

    pub fn get_currency(&self) -> String {
        let _currency = &self._currency;
        _currency.to_string()
    }

    pub fn get_client_id(&self) -> u32 {
        let client_id = &self.client_id;
        *client_id
    }

    pub fn get_client_reference(&self) -> String {
        let client_reference = &self.client_reference;
        client_reference.to_string()
    }

    pub fn get_token(&self) -> String {
        let _token = &self._token;
        _token.to_string()
    }

    pub fn get_source_id(&self) -> String {
        let source_id = &self.source_id;
        source_id.to_string()
    }

    pub fn get_purchase_nostro(&self) -> String {
        let purchase_nostro = &self.purchase_nostro;
        purchase_nostro.to_string()
    }

    pub fn get_sales_nostro(&self) -> String {
        let sales_nostro = &self.sales_nostro;
        sales_nostro.to_string()
    }

    pub fn get_option_type(&self) -> String {
        let option_type = &self.option_type;
        option_type.to_string()
    }

    pub fn get_option_start_date(&self) -> String {
        let option_start_date = &self.option_start_date;
        option_start_date.to_string()
    }

    pub fn get_jurisdiction(&self) -> String {
        let _jurisdiction = &self._jurisdiction;
        _jurisdiction.to_string()
    }

    pub fn get_rfq_id(&self) -> String {
        let rfq_id = &self.rfq_id;
        rfq_id.to_string()
    }
}

// request data

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct BookDealData {
    pub amount: f32,
    pub buySell: String,
    pub currency: String,
    pub clientId: u32,
    pub clientReference: String,
    pub token: String,
    pub sourceId: String,
    pub purchaseNostro: String,
    pub salesNostro: String,
    pub optionType: String,
    pub optionStartDate: String,
    pub jurisdiction: String,
    pub rfqId: String,
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DealData {
    pub forwardPoints: Option<f32>,
    pub quoteId: Option<String>,
    pub rfqId: Option<String>,
    pub soldCurrencyCode: Option<String>,
    pub boughtCurrencyCode: Option<String>,
    pub clientReference: Option<String>,
    pub tradeDate: Option<String>,
    pub dealRate: Option<f32>,
    pub soldCurrencyValue: Option<f32>,
    pub boughtCurrencyValue: Option<f32>,
    pub clientName: Option<String>,
    pub dealRefId: Option<String>,
    pub dealId: Option<String>,
    pub valueDate: Option<String>,
    pub errorList: Vec<String>,
    pub responseStatusCode: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DealResponseData {
    pub deal: DealData,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ErrorResponseData {
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub status: Option<u32>,
    pub detail: Option<String>,
    pub instance: Option<String>,
    pub additionalProp1: Option<String>,
    pub additionalProp2: Option<String>,
    pub additionalProp3: Option<String>,
}
