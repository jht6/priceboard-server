use serde::{Deserialize, Serialize};
use std::io;

// const STOCK_API: &str = "https://stock.xueqiu.com/v5/stock/realtime/quotec.json?symbol=";
const STOCK_API: &str = "https://stock.xueqiu.com/v5/stock/batch/quote.json?symbol=";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.75 Safari/537.36";

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
    xq_token: String,
}

impl StockFetcher {
    pub async fn new() -> Self {
        let mut fetcher = Self {
            xq_token: "".to_string(),
        };

        // 直接解包,若有错误直接panic
        fetcher.init().await.unwrap();

        fetcher
    }

    async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 获取cookie的token值
        let res = reqwest::Client::new()
            .get("https://xueqiu.com/")
            .header("User-Agent", USER_AGENT)
            .send()
            .await?;

        for v in res.headers().get_all("set-cookie").iter() {
            let s = v.to_str().unwrap();
            if s.contains("xq_a_token") {
                let list: Vec<&str> = s.split(";").collect();
                self.xq_token = list[0].to_string();
                break;
            }
        }

        Ok(())
    }

    // 参考 https://github.com/LeekHub/leek-fund/blob/master/src/explorer/stockService.ts#L382
    // Box<dyn std::error::Error>
    pub async fn get_data(
        &mut self,
        stock_code: &str,
    ) -> Result<Quote, Box<dyn std::error::Error>> {
        let json_data = reqwest::Client::new()
            .get(STOCK_API.to_owned() + stock_code)
            .header("User-Agent", USER_AGENT.to_string())
            .header("Connection", "Keep-Alive")
            .header("Accept", "text/html, application/xhtml+xml, */*")
            .header(
                "Accept-Language",
                "en-US,en;q=0.8,zh-Hans-CN;q=0.5,zh-Hans;q=0.3",
            )
            .header("Referer", "https://stock.xueqiu.com/")
            .header("Cookie", self.xq_token.to_string())
            .send()
            .await?
            .json::<ResJson>()
            // .text()
            .await?;

        println!("{json_data:#?}");

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
}
