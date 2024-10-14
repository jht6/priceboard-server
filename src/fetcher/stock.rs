use crate::common::consts::req::HEADER_USER_AGENT;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{io, sync::Mutex};
// const STOCK_HK_API: &str = "https://stock.xueqiu.com/v5/stock/batch/quote.json?symbol=";
const STOCK_HK_API: &str = "https://qt.gtimg.cn/q=";
const STOCK_API: &str = "https://hq.sinajs.cn/list=";

lazy_static! {
    static ref XQ_TOKEN: Mutex<String> = Mutex::new("".to_string());
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quote {
    pub current: f32,     // 现价
    pub percent: f32,     // 涨跌幅
    pub chg: f32,         // 涨跌价差
    pub currency: String, // 货币
}

#[derive(Clone)]
pub struct StockFetcher {
    // xq_token: String,
}

impl StockFetcher {
    pub async fn new() -> Self {
        let fetcher = Self {};

        fetcher
    }

    // 参考 https://github.com/LeekHub/leek-fund/blob/master/src/explorer/stockService.ts#L382
    pub async fn get_data(&self, stock_code: &str) -> Result<Quote, Box<dyn std::error::Error>> {
        if stock_code.starts_with("hk") {
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
        let stock_code = "r_".to_owned() + _stock_code;
        let url = STOCK_HK_API.to_owned() + &stock_code + "&fmt=json";
        let json_data = reqwest::Client::new()
            .get(url)
            .header("User-Agent", HEADER_USER_AGENT.to_string())
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .send()
            .await?
            .text_with_charset("GBK")
            .await?;

        // println!("{json_data:#?}");

        let obj: serde_json::Value = serde_json::from_str(&json_data).unwrap();
        //
        let list_str = serde_json::to_string(&obj[stock_code]).unwrap();
        let arr: Vec<String> = serde_json::from_str(&list_str).unwrap();

        // println!("{:?}", arr);

        Ok(Quote {
            current: arr[3].parse::<f32>().unwrap(),
            percent: arr[32].parse::<f32>().unwrap(),
            chg: arr[31].parse::<f32>().unwrap(),
            currency: "HKD".to_string(),
        })
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
