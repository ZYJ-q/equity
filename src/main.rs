use std::collections::VecDeque;
use std::{collections::HashMap, fs, time::Duration};
// use chrono::Local;
use chrono:: Utc;
use log::{info, warn};
use serde_json::{Map, Value};
// use tokio::{sync::broadcast::{self, Receiver}};
use equity::adapters::binance::futures::http::actions::BinanceFuturesApi;
use equity::base::ssh::SshClient;
use equity::base::wxbot::WxbotHttpClient;
use equity::actors::*;
// use test_alarm::models::http_data::*;

#[warn(unused_mut, unused_variables, dead_code)]
async fn real_time(
    // binance_futures_api: BinanceFuturesApi,
    binance: &Vec<Value>,
    symbols: &Vec<Value>,
    mut ssh_api: SshClient,
    wx_robot: WxbotHttpClient,
    ori_fund: f64,
) {
    //rece: &mut Receiver<&str>){
    info!("get ready for real time loop");
    let running = true;
    // let mut end = 6;

    // 每个品种的上一个trade_id
    let mut last_trade_ids: HashMap<String, u64> = HashMap::new();
    for symbol_v in symbols {
        let symbol = String::from(symbol_v.as_str().unwrap());
        let symbol = format!("{}USDT", symbol);
        last_trade_ids.insert(symbol, 0);
    }

    // 权益数据统计
    

    // 净值数据
    // let mut net_worth_histories: VecDeque<Value> = VecDeque::new();

    info!("begin real time loop");
    // 监控循环
    loop {
        info!("again");
        // json对象
        // let mut response: Map<String, Value> = Map::new();
        // let mut json_data: Map<String, Value> = Map::new();
        let mut map: Map<String, Value> = Map::new();
        let mut equity_histories: VecDeque<Value> = VecDeque::new();
        

        // 监控服务器状态
        info!("server process");
        // let mut server_status: VecDeque<Value> = VecDeque::new();
        // let mut server_process: Map<String, Value> = Map::new();
        // print!("判断是true还是false {}", ssh_api.search_py_ps());
        // match ssh_api.search_py_ps() {
        //     true => {
        //         if !running {
        //             running = true;
        //             print!("改变running的值{}", running);
        //             // let sender = "程序开启";
        //             // let content = format!("process name: {}", ssh_api.get_root_name());
        //             // wx_robot.send_text(sender, &content).await;
        //         }
        //         server_process.insert(String::from("status"), Value::from("running"));
        //         server_process.insert(String::from("info"), Value::from(""));
        //     }
        //     false => {
        //         server_process.insert(String::from("status"), Value::from("stopped"));
        //         let mut info = ssh_api.download_log();
        //         if running {
        //             running = false;
        //             // let sender = "程序停止";
        //             let content;
        //             if info == "" {
        //                 content = format!("{}: 未找到错误，请查看日志", ssh_api.get_root_name());
        //             }else {
        //                 content = format!("{}: {}", ssh_api.get_root_name(), &info);
        //             }
        //             // wx_robot.send_text(sender, &content).await;
        //             info = content;
        //         }
        //         server_process.insert(String::from("info"), Value::from(info));
        //     }
        // }
        // map.insert(String::from("server"), Value::from(server_process));

        for f_config in binance {
            let mut equity_map: Map<String, Value> = Map::new();
        let now = Utc::now();
        let date = format!("{}", now.format("%Y/%m/%d %H:%M:%S"));
            let binance_config = f_config.as_object().unwrap();
            let binance_futures_api=BinanceFuturesApi::new(
                binance_config
                    .get("base_url")
                    .unwrap()
                    .as_str()
                    .unwrap(),
                binance_config
                    .get("api_key")
                    .unwrap()
                    .as_str()
                    .unwrap(),
                binance_config
                    .get("secret_key")
                    .unwrap()
                    .as_str()
                    .unwrap(),
            );
            let name = binance_config.get("name").unwrap().as_str().unwrap();

            if let Some(data) = binance_futures_api.account(None).await {
                let value: Value = serde_json::from_str(&data).unwrap();
                let assets = value.as_object().unwrap().get("assets")
            .unwrap().as_array().unwrap();
            let mut new_total_equity = 0.00;
            let mut best_price = 0.00;
            for a in assets {
                let obj = a.as_object().unwrap();
                let wallet_balance: f64 = obj.get("walletBalance").unwrap().as_str().unwrap().parse().unwrap();
                let symbol = obj.get("asset").unwrap().as_str().unwrap();
    
                if wallet_balance != 0.00 {
                    if symbol == "ETH" {
                        let asset = format!("{}USDT", symbol);
                        if let Some(data) = binance_futures_api.get_klines(&asset).await {
                            let v: Value = serde_json::from_str(&data).unwrap();
                            let price_obj = v.as_object().unwrap();
                            let price:f64 = price_obj.get("price").unwrap().as_str().unwrap().parse().unwrap();
                            best_price = price;
                            let new_price = wallet_balance * price;
                            // new_total_balance += new_price;
                            new_total_equity += new_price;
                        }
                    }
    
                    let cross_un_pnl: f64 = obj.get("crossUnPnl").unwrap().as_str().unwrap().parse().unwrap();
                    let pnl = cross_un_pnl + wallet_balance;
                    // new_total_balance += wallet_balance;
                    new_total_equity += pnl;
                }
            }
            // 权益
            let new_total_equity_eth: f64 = ((new_total_equity / best_price) - 40.00) * best_price;
            equity_map.insert(String::from("time"), Value::from(date));
            equity_map.insert(String::from("name"), Value::from(name));
            equity_map.insert(String::from("equity_eth"), Value::from(new_total_equity_eth.to_string()));
            equity_map.insert(String::from("equity"), Value::from(new_total_equity.to_string()));
            equity_histories.push_back(Value::from(equity_map));
            }
    
            

        }
        let res = trade_mapper::TradeMapper::insert_equity(Vec::from(equity_histories.clone()));
        println!("插入权益数据{}, 数据{:?}", res, Vec::from(equity_histories.clone()));


        // 获取账户信息
        

        



        // 等待下次执行
        info!("waiting for next real time task...({})", 4320000 * 10);
        tokio::time::delay_for(Duration::from_millis(4320000 * 10)).await;
    }
}

#[warn(unused_mut, unused_variables)]
#[tokio::main]
async fn main() {
    // 日志
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();

    init();
    // let time = format!("{}", Local::now().format("%Y/%m/%d %H:%M:%S"));

    // 测试用api
    // let api_key="JwYo1CffkOLqmv2sC3Qhe2Qu5GgzbeLVw2BxWB5HgK6tnmc8yGfkzLuDImBgDkXm";
    // let api_secret="7FtQARZqM2PDgIZ5plr3nwEVYBXXbvmSuvmpf6Viz9e7Cq2B87grRTG3VZQiEC5C";

    // 连接数据库
    // let config_db: Value =
    //     serde_json::from_str(&fs::read_to_string("./configs/database.json").unwrap()).unwrap();

    // 读取配置
    let config: Value = serde_json::from_str(
        &fs::read_to_string("./configs/total.json").expect("Unable to read file"),
    )
    .expect("Unable to parse");

    // 任务间通信信道
    // let (send, mut rece) = broadcast::channel(32);

    // 创建任务
    let real_time_handle = tokio::spawn(async move {
        // let mut futures_config: Map<String, Value> = Map::new();
        // let mut servers_config = Map::new();
        let binance_config = config.get("Binance").unwrap();
        let binance_future_config = binance_config.get("futures").unwrap().as_array().unwrap();
        let server_config = config.get("Server").unwrap();
        let symbols = config.get("Symbols").unwrap().as_array().unwrap();
        let key = config.get("Alarm").unwrap().get("webhook").unwrap().as_str().unwrap();
        // info!("获取key");
        let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
        wxbot.push_str(key);
        info!("wxbot  {}", wxbot);
        let wx_robot = WxbotHttpClient::new(&wxbot);
        info!("preparing...");

        // for s_config in server_config{
        //     let obj = s_config.as_object().unwrap(); 
        //     let host = obj.get("host").unwrap().as_str().unwrap();
        //     let port = obj.get("port").unwrap().as_str().unwrap();
        //     let username = obj.get("username").unwrap().as_str().unwrap();
        //     let password = obj.get("password").unwrap().as_str().unwrap();
        //     let root_path = obj.get("root_path").unwrap().as_str().unwrap();
        //     let root_name = obj.get("root_name").unwrap().as_str().unwrap();
        //     servers_config.insert(String::from("host"), Value::from(host));
        //     servers_config.insert(String::from("port"), Value::from(port));
        //     servers_config.insert(String::from("username"), Value::from(username));
        //     servers_config.insert(String::from("password"), Value::from(password));
        //     servers_config.insert(String::from("root_path"), Value::from(root_path));
        //     servers_config.insert(String::from("root_name"), Value::from(root_name));
        // }
        
        
        
        let ssh_api = SshClient::new(
            server_config.get("host").unwrap().as_str().unwrap(),
            server_config.get("port").unwrap().as_str().unwrap(),
            server_config.get("username").unwrap().as_str().unwrap(),
            server_config.get("password").unwrap().as_str().unwrap(),
            server_config.get("root_path").unwrap().as_str().unwrap(),
            server_config.get("root_name").unwrap().as_str().unwrap(),
        );
        

        
        // for f_config in binance_future_config{
        //     let obj = f_config.as_object().unwrap(); 
        //     let base_url = obj.get("base_url").unwrap().as_str().unwrap();
        //     let api_key = obj.get("api_key").unwrap().as_str().unwrap();
        //     let secret_key = obj.get("secret_key").unwrap().as_str().unwrap();
        //     futures_config.insert(String::from("base_url"), Value::from(base_url));
        //     futures_config.insert(String::from("api_key"), Value::from(api_key));
        //     futures_config.insert(String::from("secret_key"), Value::from(secret_key));
        // }

        info!("created ssh client");
        // let binance_futures_api=BinanceFuturesApi::new(
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("base_url")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("api_key")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("secret_key")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        // );

        
        info!("created http client");

            real_time(binance_future_config, symbols, ssh_api, wx_robot, 500.0).await;
        
    });

    // 开始任务
    info!("alarm begin(binance account)");
    real_time_handle.await.unwrap();
    info!("alarm done");
}
