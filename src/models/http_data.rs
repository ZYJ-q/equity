use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug, Clone)]
pub struct TradeRe {
    pub id: u64,
    pub symbol: String,
    pub order_id: u64,
    pub tra_id: u64,
    pub commission: String,
    pub time: i64,
    pub is_maker: bool,
    pub position_side: String,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
    pub realized_pnl: String,
    pub side: String,
}