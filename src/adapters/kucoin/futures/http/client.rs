use super::super::super::super::super::base::http::HttpClient;
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use hmac::{Hmac, Mac};
use itertools::Itertools;
#[allow(unused_imports)]
use log::{error, info};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Method, Response, StatusCode};
use serde_json::value::Value;
use sha2::Sha256;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct KucoinHttpClient {
    http_client: HttpClient,
    base_url: String,
    api_key: String,
    api_secret: String,
    passphrase: String,
    isv1api: bool,
}

#[allow(dead_code)]
impl KucoinHttpClient {
    pub fn new(
        base_url: &str,
        api_key: &str,
        api_secret: &str,
        passphrase: &str,
        isv1api: bool,
    ) -> Self {
        let http_client = HttpClient::new();
        Self {
            http_client: http_client,
            base_url: String::from(base_url),
            api_key: String::from(api_key),
            api_secret: String::from(api_secret),
            passphrase: String::from(passphrase),
            isv1api: isv1api,
        }
    }

    async fn package_request(
        &self,
        method: &Method,
        url: &str,
        need_sign: bool,
        params: &HashMap<String, Value>,
    ) -> Option<Response> {
        // env_logger::init();
        let mut uri_path = String::from(url);
        let mut uri = String::from(url);
        let mut data_json = String::new();

        if method == Method::GET || method == Method::DELETE {
            if !params.is_empty() {
                let mut strl: Vec<String> = Vec::new();
                for key in params.keys().sorted() {
                    strl.push(format!("{}={}", key, params.get(key).unwrap().to_string()));
                }
                for i in 0..strl.len() {
                    if i == 0 {
                        data_json.push_str(&strl[i]);
                    } else {
                        data_json.push('&');
                        data_json.push_str(&strl[i]);
                    }
                }
                uri = format!("{}?{}", &uri, &data_json);
                uri_path = String::from(&uri);
            }
        } else {
            if !params.is_empty() {
                match serde_json::to_string(&params) {
                    Ok(result) => data_json = result,
                    Err(e) => error!("error on parase params: {}", e),
                }
                uri_path = format!("{}{}", &uri, &data_json);
            }
        }
        let mut headers = HeaderMap::new();
        if need_sign {
            let now_time = Utc::now().timestamp_millis();

            let str_to_sign = format!("{}{}{}", &now_time.to_string(), &method.as_str(), &uri_path);
            // println!("{str_to_sign}");

            let mut hmac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes()).unwrap();
            hmac.update(str_to_sign.as_bytes());
            let sign = hmac.finalize().into_bytes();
            if self.isv1api {
                headers.insert(
                    "KC-API-SIGN",
                    HeaderValue::try_from(general_purpose::STANDARD.encode(sign)).unwrap(),
                );
                headers.insert("KC-API-TIMESTAMP", now_time.to_string().parse().unwrap());
                headers.insert("KC-API-KEY", self.api_key.parse().unwrap());
                headers.insert("KC-API-PASSPHRASE", self.passphrase.parse().unwrap());
                headers.insert("Content-Type", "application/json".parse().unwrap());
            } else {
                let mut hmac_p =
                    Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes()).unwrap();
                hmac_p.update(self.passphrase.as_bytes());
                let passphrase = hmac_p.finalize().into_bytes();
                headers.insert(
                    "KC-API-SIGN",
                    HeaderValue::try_from(general_purpose::STANDARD.encode(sign)).unwrap(),
                );
                headers.insert("KC-API-TIMESTAMP", now_time.to_string().parse().unwrap());
                headers.insert("KC-API-KEY", self.api_key.parse().unwrap());
                headers.insert(
                    "KC-API-PASSPHRASE",
                    general_purpose::STANDARD
                        .encode(passphrase)
                        .parse()
                        .unwrap(),
                );
                headers.insert("Content-Type", "application/json".parse().unwrap());
                headers.insert("KC-API-KEY-VERSION", "2".parse().unwrap());
            }
        }
        headers.insert("User-Agent", "nautilus_alarm".parse().unwrap());
        let url = format!("{}{}", self.base_url, uri);

        return self
            .http_client
            .send_request(&method.as_str(), &url, headers, &data_json)
            .await;
    }

    pub async fn send(
        &self,
        method: Method,
        url: &str,
        need_sign: bool,
        params: &HashMap<String, Value>,
    ) -> Option<HashMap<String, Value>> {
        // let data: HashMap<String, Value> = HashMap::new();
        if let Some(response) = self.package_request(&method, url, need_sign, params).await {
            if response.status() == StatusCode::OK {
                match response.json().await {
                    Ok(response_data) => match response_data {
                        Some(data) => {
                            return Some(data);
                        }
                        None => {
                            error!("error on parse json: {:?}", response_data);
                            return None;
                        }
                    },
                    Err(e) => {
                        error!("error on parse response: {}", e);
                        return None;
                    }
                }
            } else {
                panic!("{}-{}", response.status(), response.text().await.unwrap());
            }
        } else {
            panic!(
                "none response: {}-{}-{}-{:?}",
                &method.as_str(),
                url,
                need_sign,
                params
            );
        }
    }

    pub fn check_response_data(&self, data_s: Option<HashMap<String, Value>>) -> Option<Value> {
        match data_s {
            Some(data) => {
                if !data.is_empty() {
                    match data.get("code") {
                        Some(code) => {
                            if code.to_string() == "200000" {
                                match data.get("data") {
                                    Some(data_r) => {
                                        return Some(data_r.clone());
                                    }
                                    None => {
                                        info!("response: {}-{:?}", code, data);
                                        return None;
                                    }
                                }
                            } else if code.to_string() == "100001" {
                                info!("response: {}-{:?}", code, data);
                                return None;
                            } else {
                                panic!("invalid code: {}", code);
                            }
                        }
                        None => {
                            panic!("no code: {:?}", data);
                        }
                    }
                } else {
                    panic!("response is empty");
                }
            }
            None => {
                panic!("handle response failed")
            }
        }
    }
}
