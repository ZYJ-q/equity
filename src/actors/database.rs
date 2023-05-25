// use std::collections::HashMap;
use once_cell::sync::OnceCell;

// use actix_web::web;
// use mysql::prelude::*;
use mysql::{Pool, PooledConn};
use tracing::{instrument, info};


static DB_POOL: OnceCell<Pool> = OnceCell::new();


// 连接数据库
#[instrument]
pub fn create_pool(db_url: &str) {
    // let user = config_db.get("user").unwrap();
    // let password = config_db.get("password").unwrap();
    // let host = config_db.get("host").unwrap();
    // let port = config_db.get("port").unwrap();
    // let database = config_db.get("database").unwrap();
    // let url = format!(
    //     "mysql://{}:{}@{}:{}/{}",
    //     user, password, host, port, database
    // );
    info!("初始化数据线程池------开始-------");
    DB_POOL.set(mysql::Pool::new(&db_url).expect(&format!("Error connecting to {}", &db_url)))
           .unwrap_or_else(|_| { info!("try insert pool cell failure!") });
    info!("初始化数据线程池------结束-------");
    // let pool = Pool::new(url).unwrap();
    // return pool;
}

#[instrument]
pub fn get_connect() -> PooledConn {
  info!("从链接池获取数据库链接----------开始----------");
  let conn = DB_POOL.get().expect("Error get pool from OneCell<Pool>").get_conn().expect("Error get_connect from db pool");
  info!("从链接池获取数据库链接----------结束----------");
  conn
}



