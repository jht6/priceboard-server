use crate::common::consts::req::HEADER_USER_AGENT;
use chrono::Local;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{io, sync::Mutex, thread, time::Duration};
const STOCK_HK_API: &str = "https://stock.xueqiu.com/v5/stock/batch/quote.json?symbol=";
const STOCK_API: &str = "https://hq.sinajs.cn/list=";
const FETCH_TOKEN_INTERVAL_SECS: u64 = 60;

lazy_static! {
    static ref XQ_TOKEN: Mutex<String> = Mutex::new("".to_string());
}

// ------------------------雪球API json结构----------------------------
#[derive(Serialize, Deserialize, Debug)]
struct ResJson {
    data: Data,
    error_code: i32,
    error_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    items: Vec<DataItem>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataItem {
    quote: Quote,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quote {
    pub current: f32,     // 现价
    pub percent: f32,     // 涨跌幅
    pub chg: f32,         // 涨跌价差
    pub currency: String, // 货币
}
// ------------------------------------------------------------------

#[derive(Clone)]
pub struct StockFetcher {
    // xq_token: String,
}

impl StockFetcher {
    pub async fn new() -> Self {
        let fetcher = Self {};

        // 直接解包,若有错误直接panic
        Self::update_token().await.unwrap();

        tokio::task::spawn(async {
            loop {
                thread::sleep(Duration::from_secs(FETCH_TOKEN_INTERVAL_SECS));
                Self::update_token().await.unwrap();
            }
        });

        fetcher
    }

    fn set_token(token: String) {
        let mut t = XQ_TOKEN.lock().unwrap();
        *t = token;
    }

    async fn fetch_token() -> Result<String, Box<dyn std::error::Error>> {
        let res = reqwest::Client::new()
            .get("https://xueqiu.com/")
            .header("User-Agent", HEADER_USER_AGENT)
            .send()
            .await?;

        for v in res.headers().get_all("set-cookie").iter() {
            let s = v.to_str().unwrap();
            if s.contains("xq_a_token") {
                let list: Vec<&str> = s.split(";").collect();
                return Ok(list[0].to_string());
            }
        }

        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Cannot get xq_token",
        )))
    }

    pub async fn update_token() -> Result<(), Box<dyn std::error::Error>> {
        let token = Self::fetch_token().await.unwrap();
        let old_token = XQ_TOKEN.lock().unwrap().clone();

        let now = Local::now();

        if token != old_token {
            Self::set_token(token.clone());

            if old_token != "" {
                println!(
                    "{} xq_token is changed: {}",
                    now.format("%Y-%m-%d %H:%M:%S"),
                    &token
                );
            }
        }

        Ok(())
    }

    // 参考 https://github.com/LeekHub/leek-fund/blob/master/src/explorer/stockService.ts#L382
    pub async fn get_data(&self, stock_code: &str) -> Result<Quote, Box<dyn std::error::Error>> {
        if stock_code.starts_with("HK") {
            let ret = self.get_hk_data(stock_code).await;
            ret
        } else {
            let ret = self.get_cn_us_data(stock_code).await;
            ret
        }
    }

    pub async fn get_hk_data(
        &self,
        _stock_code: &str,
    ) -> Result<Quote, Box<dyn std::error::Error>> {
        let stock_code = &_stock_code[2..];
        let t = XQ_TOKEN.lock().unwrap();
        let token = t.clone();
        drop(t); // 释放锁

        let json_data = reqwest::Client::new()
            .get(STOCK_HK_API.to_owned() + stock_code)
            .header("User-Agent", HEADER_USER_AGENT.to_string())
            .header("Connection", "Keep-Alive")
            .header("Accept", "text/html, application/xhtml+xml, */*")
            .header(
                "Accept-Language",
                "en-US,en;q=0.8,zh-Hans-CN;q=0.5,zh-Hans;q=0.3",
            )
            .header("Referer", "https://stock.xueqiu.com/")
            .header("Cookie", token)
            .send()
            .await?
            .json::<ResJson>()
            // .text()
            .await?;

        // println!("{json_data:#?}");

        if json_data.error_code != 0 {
            Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                json_data.error_description,
            )))
        } else {
            let x = &json_data.data.items[0].quote;
            Ok(x.clone())
        }
    }

    pub async fn get_cn_us_data(
        &self,
        stock_code: &str,
    ) -> Result<Quote, Box<dyn std::error::Error>> {
        let rawdata = reqwest::Client::new()
            .get(STOCK_API.to_owned() + stock_code)
            .header("User-Agent", HEADER_USER_AGENT.to_string())
            .header("Connection", "Keep-Alive")
            .header("Accept", "text/html, application/xhtml+xml, */*")
            .header(
                "Accept-Language",
                "en-US,en;q=0.8,zh-Hans-CN;q=0.5,zh-Hans;q=0.3",
            )
            .header("Referer", "https://finance.sina.com.cn/")
            .send()
            .await?
            // .bytes()
            .text()
            .await?;

        if rawdata.contains("FAILED") {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, "")));
        }

        let re = Regex::new(r#""([^"]+)""#).unwrap();
        let data = re
            .captures(&rawdata)
            .and_then(|cap| cap.get(1).map(|quoted_str| quoted_str.as_str()))
            .unwrap();
        let list: Vec<&str> = data.split(",").collect();

        // println!("extracted data: {:?}", data);

        if stock_code.starts_with("usr_") {
            // 美股
            let current: f32 = list[1].parse().unwrap();
            let percent: f32 = list[2].parse().unwrap();
            let chg: f32 = list[4].parse().unwrap();
            return Ok(Quote {
                current,
                percent,
                chg,
                currency: "USD".to_string(),
            });
        } else {
            // 其他默认国内股, 包括: sh/sz/bj
            let yesterday: f32 = list[2].parse().unwrap();
            let current: f32 = list[3].parse().unwrap();
            let percent = (current - yesterday) / yesterday * 100.0;
            let chg: f32 = current - yesterday;
            return Ok(Quote {
                current,
                percent,
                chg,
                currency: "CNY".to_string(),
            });
        }
    }
}
