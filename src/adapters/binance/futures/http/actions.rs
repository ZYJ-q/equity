use chrono::Utc;
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use super::client::BinanceHttpClient;
use chrono::Local;
use chrono::{DateTime, NaiveDateTime};

pub struct BinanceFuturesApi {
    client: BinanceHttpClient,
}

impl BinanceFuturesApi {
    pub fn new(base_url: &str, api_key: &str, api_secret: &str) -> Self {
        let client = BinanceHttpClient::new(base_url, api_key, api_secret);
        Self { client: client }
    }

    pub async fn account(&self, recv_window: Option<u8>) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        match recv_window {
            Some(recvwindow) => {
                params.insert(
                    String::from("recvWindow"),
                    recvwindow.to_string().parse().unwrap(),
                );
            }
            None => {}
        }
        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v2/account", true, &mut params)
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

    pub async fn total_account(&self, recv_window: Option<u8>) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        match recv_window {
            Some(recvwindow) => {
                params.insert(
                    String::from("recvWindow"),
                    recvwindow.to_string().parse().unwrap(),
                );
            }
            None => {}
        }
        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v2/balance", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        match res_data {
            Some(data) => {
                println!("账户总余额{:?}", data);
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    // 获取最新价格
    pub async fn get_klines(&self, symbol: &str) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(String::from("symbol"), Value::from(symbol));

        // let now_time = Utc::now().timestamp_millis();
        // params.insert(String::from("interval"), Value::from("15m"));

        let response = self
            .client
            .send(Method::GET, "/fapi/v1/ticker/price", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        match res_data {
            Some(data) => {
                // print!("K线数据{}", data);
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }
    
    pub async fn position_risk(&self, symbol: Option<&str>) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        match symbol {
            Some(symbol_s) => {
                params.insert(
                    String::from("symbol"),
                    String::from(symbol_s).parse().unwrap(),
                );
            }
            None => {}
        }
        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v2/positionRisk", true, &mut params)
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

    pub async fn trade_hiostory(&self, symbol: &str, end: &i64) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(String::from("symbol"), Value::from(symbol));



        println!("参数值:{:?}, {}", end, symbol);



        let now_time = Utc::now().timestamp_millis();
        let time = Local::now().timestamp_millis();
        let end_datetime: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp_millis(time - 1000*60*60*24 * end).unwrap(), Utc,);
        let start_datetime: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp_millis(time - 1000*60*60*24 * (end +  1)).unwrap(), Utc,);
        let end_time= format!("{} ", end_datetime.format("%Y-%m-%d %H:%M:%S"));
        let start_time = format!("{} ", start_datetime.format("%Y-%m-%d %H:%M:%S"));
        println!("end时间整点{:?}", end_time);
        println!("start时间整点{}", start_time);
        params.insert(String::from("timestamp"), Value::from(now_time));
        params.insert(String::from("startTime"), Value::from(time - 1000*60*60*24 * (end+1)));
        params.insert(String::from("endTime"), Value::from(time - 1000*60*60*24 * end));
        // if end == &0 {
        //     params.insert(String::from("startTime"), Value::from(time - 1000*60*60*24 * (end+1)));
        //     params.insert(String::from("endTime"), Value::from(time - 1000*60*60*24 * end));
        // } else {
        //     params.insert(String::from("startTime"), Value::from(.timestamp_millis()));
        //     params.insert(String::from("endTime"), Value::from());
        // }

        // println!("endTime:{:?}", params.insert(String::from("endTime"), Value::from(now_time - 1000*60*60*24 * end)));
        // println!("startTime:{:?}", params.insert(String::from("startTime"), Value::from(now_time - 1000*60*60*24 * (end+1))));


        let response = self
            .client
            .send(Method::GET, "/fapi/v1/userTrades", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        

        match res_data {
            Some(data) => {
                // print!("历史成交数据{}", data);
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    pub async fn position(&self, symbol: Option<&str>) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        match symbol {
            Some(s) => {
                params.insert(String::from("symbol"), Value::from(s));

            }
            None => {}
        }

        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v2/positionRisk", true, &mut params)
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
