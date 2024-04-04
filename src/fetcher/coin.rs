use crate::common::consts::req::HEADER_USER_AGENT;
use serde::{Deserialize, Serialize};
use std::io;

const COIN_API: &str = "https://m.bi123.co/crypto-web/wiki/spot";

// 请求体
#[derive(Serialize)]
struct ApiReqBody {
    contract: String, // BTC
    exchange: String, // Binance
    symbol: String,   // BTC
}

// 响应体
#[derive(Serialize, Deserialize)]
struct ResJson {
    code: i32,
    msg: String,
    success: bool,
    data: ResData,
}

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct ResData {
    pub price: f32,                // 当前币价
    pub priceDayChange: f32,       // 涨跌幅
    pub priceDayChangeAmount: f32, // 涨跌价差
}

#[derive(Clone)]
pub struct CoinFetcher {}

impl CoinFetcher {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_data(&self, coin_name: &str) -> Result<ResData, Box<dyn std::error::Error>> {
        let req_json = ApiReqBody {
            contract: coin_name.to_string(), // BTC
            exchange: "Binance".to_string(), // Binance
            symbol: coin_name.to_string(),   // BTC
        };
        let json_data = reqwest::Client::new()
            .post(COIN_API.to_owned())
            .header("User-Agent", HEADER_USER_AGENT.to_string())
            // .header("Connection", "Keep-Alive")
            .header("Accept", "application/json")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .header("Referer", "https://m.bi123.co/")
            // .header("Cookie", self.xq_token.to_string())
            .json(&req_json)
            .send()
            .await?
            .json::<ResJson>()
            // .text()
            .await?;

        // println!("{json_data:#?}");

        if !json_data.success {
            Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                json_data.msg,
            )))
        } else {
            let x = &json_data.data;
            Ok(x.clone())
        }
    }
}
