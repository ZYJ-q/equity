use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use super::client::ByBitHttpClient;

pub struct ByBitFuturesApi {
    client: ByBitHttpClient,
}

impl ByBitFuturesApi {
    pub fn new(
        base_url: &str,
        api_key: &str,
        api_secret: &str,
    ) -> Self {
        let client = ByBitHttpClient::new(base_url, api_key, api_secret);
        Self { client }
    }

    pub async fn get_account_overview(&self, account_type: Option<&str>) -> Option<String> {
        // let my_currency = String::from(currency.unwrap_or("USDT"));

        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(
            String::from("accountType"),
            Value::from(account_type),
        );

        let response = self
            .client
            .send(Method::GET, "/v5/account/wallet-balance", true,&mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        // println!("账户信息11111111111111111111{:?}", res_data);

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
