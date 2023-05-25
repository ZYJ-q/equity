use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub acc_id: u64,
    pub acc_name: String,
    pub acc_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccProd {
    pub ap_id: u64,
    pub acc_id: u64,
    pub prod_id: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub prod_id: u64,
    pub prod_name: String,
    pub weixin_id: u64,
    pub prog_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Active {
    pub acc_id: u64,
    pub token: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trader {
    pub tra_id: u64,
    pub tra_venue: String,
    pub ori_balance: String,
    pub tra_currency: String,
    pub api_key: String,
    pub secret_key: String,
    pub other_keys: String,
    pub r#type: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    pub th_id: u64,
    pub tra_symbol: String,
    pub tra_order_id: u64,
    // pub tra_id: u64,
    pub tra_commision: String,
    pub tra_time: String,
    pub is_maker: String,
    pub position_side: String,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
    pub realized_pnl: String,
    pub side: String,
}