use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use super::client::KucoinHttpClient;

pub struct KucoinFuturesApi {
    client: KucoinHttpClient,
}

impl KucoinFuturesApi {
    pub fn new(
        base_url: &str,
        api_key: &str,
        api_secret: &str,
        passphrase: &str,
        isv1api: bool,
    ) -> Self {
        let client = KucoinHttpClient::new(base_url, api_key, api_secret, passphrase, isv1api);
        Self { client: client }
    }

    pub async fn get_account_overview(&self, currency: Option<&str>) -> Option<Value> {
        let my_currency = String::from(currency.unwrap_or("USDT"));

        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(
            String::from("currency"),
            serde_json::to_value(my_currency).unwrap(),
        );

        let response = self
            .client
            .send(Method::GET, "/api/v1/account-overview", true, &params)
            .await;

        let res_data = self.client.check_response_data(response);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }
}
