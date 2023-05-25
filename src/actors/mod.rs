mod database;
pub mod trade_mapper;

// use super::models::http_data;
// use super::models::db_data;

const DB_URL: &str = "mysql://root:uAYxyg2S=6TG@localhost:3306/monitor";

pub fn init() {
  // 初始化连接池
  database::create_pool(DB_URL);
}

