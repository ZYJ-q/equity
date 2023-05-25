pub struct TradeMapper;
pub struct PositionMapper;

pub struct NetWorkMapper;
// use super::http_data::TradeRe;
use crate::actors::database::get_connect;
// use log::info;
use mysql::*;
use mysql::prelude::*;
use serde_json::Value;
// use super::db_data::Trade;


impl TradeMapper {
  // 插入数据
  pub fn insert_trade(trades:Vec<Value>) -> bool {
    // 连接数据库
    let mut conn = get_connect();
    // let query_id = conn.exec_first(, params)

    let flag = conn.exec_batch(
      r"INSERT IGNORE INTO trate_histories_6 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
      VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)",
      trades.iter().map(|p| params! {
        "th_id" => &p["th_id"],
        "tra_symbol" => &p["tra_symbol"],
        "tra_order_id" => &p["tra_order_id"],
        // "tra_id" => &p["tra_id"],
        "tra_commision" => &p["tra_commision"],
        "tra_time" => &p["tra_time"],
        "is_maker" => &p["is_maker"].to_string(),
        "position_side" => &p["position_side"],
        "price" => &p["price"],
        "qty" => &p["qty"],
        "quote_qty" => &p["quote_qty"],
        "realized_pnl" => &p["realized_pnl"],
        "side" => &p["side"],
      })
    );

  // let um1 = conn.query_map(
  // "select * from trate_histories",
  // |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
  //     Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
  // }
  // ).unwrap();

  // println!("查询到的数据{:?}", um1);

    match flag {
      Ok(_c) => {
        println!("insert success!");
        return true;
      },
      Err(e) => {
        eprintln!("error:{}", e);
        return false;
      }
    }
  }
}

impl PositionMapper {
  pub fn insert_position(position: Vec<Value>) -> bool {
    let mut coon = get_connect();

    let positions = coon.exec_batch(
      r"INSERT IGNORE INTO position_histories_6 (symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price)
      VALUES (:symbol, :position_amt, :position_side, :time, :entry_price, :un_realized_profit, :tra_id, :leverage, :mark_price)",
      position.iter().map(|p| params! {
        "symbol" => &p["symbol"],
        "position_amt" => &p["position_amt"],
        "position_side" => &p["position_side"],
        "time" => &p["time"],
        "entry_price" => &p["entry_price"],
        "un_realized_profit" => &p["un_realized_profit"],
        "tra_id" => &p["tra_id"],
        "leverage" => &p["leverage"],
        "mark_price" => &p["mark_price"],
      })
    );

    match positions {
      Ok(_c) => {
        println!("insert position success");
        return true;
      },
      Err(e) => {
        eprintln!("error:{}", e);
        return false;
      }
    }
  } 
}


// 更新服务器状态

// 添加净值数据

impl NetWorkMapper {
  pub fn insert_net_worth(net_worth: Vec<Value>) -> bool {
    let mut coon = get_connect();

    let net_worths = coon.exec_batch(
      r"INSERT IGNORE INTO networth_histories_6 (time, total_equity)
      VALUES (:time, :total_equity)",
      net_worth.iter().map(|p| params! {
        "time" => &p["time"],
        "total_equity" => &p["total_equity"],
      })
    );

    match net_worths {
      Ok(_c) => {
        println!("insert position success");
        return true;
      },
      Err(e) => {
        eprintln!("error:{}", e);
        return false;
      }
    }
  } 
}











