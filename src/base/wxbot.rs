use std::collections::HashMap;

use reqwest::header::HeaderMap;
use serde_json::{Value, Map};

use super::http::HttpClient;

pub struct WxbotHttpClient {
    http_client: HttpClient,
    wxbot: String,
}

#[allow(dead_code)]
impl WxbotHttpClient {
    pub fn new(wxbot: &str) -> Self {
        let http_client = HttpClient::new();
        Self {
            http_client: http_client,
            wxbot: String::from(wxbot),
        }
    }

    fn package_text(&self, sender: &str, content: &str) -> String {
        return format!("{sender}:\n{content}")
    }

    pub async fn send_text(&self, sender: &str, content: &str) {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(String::from("msgtype"), Value::from("text"));
        let mut map: Map<String, Value> = Map::new();
        map.insert(String::from("content"), Value::from(self.package_text(sender, content)));
        params.insert(String::from("text"), Value::from(map));
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        println!("POST {} {:?} {}", &self.wxbot, headers, &serde_json::to_string(&params).unwrap());
        self.http_client.send_request("POST", &self.wxbot, headers, &serde_json::to_string(&params).unwrap()).await;
    }
}