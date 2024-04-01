use serde::{Deserialize, Serialize};

// const STOCK_API: &str = "https://stock.xueqiu.com/v5/stock/realtime/quotec.json?symbol=";
const STOCK_API: &str = "https://stock.xueqiu.com/v5/stock/batch/quote.json?symbol=";

#[derive(Serialize, Deserialize, Debug)]
struct DataItem {
    current: f32, // 现价
    percent: f32, // 涨跌幅
    chg: f32,     // 涨跌价差
}

#[derive(Serialize, Deserialize, Debug)]
struct ResJson {
    data: Vec<DataItem>,
    error_code: i32,
    error_description: String,
}

pub struct StockFetcher {
    xq_token: String,
}

impl StockFetcher {
    pub fn new() -> Self {
        let sf = Self {
            xq_token: "".to_string(),
        };

        sf
    }

    // 获取雪球token
    pub async fn get_token(&mut self) -> Result<&String, Box<dyn std::error::Error>> {
        if self.xq_token != "" {
            return Ok(&self.xq_token);
        }

        let res = reqwest::Client::new()
            .get("https://xueqiu.com/")
            .header("User-Agent","Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.75 Safari/537.36")
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

        Ok(&self.xq_token)
    }

    // 参考 https://github.com/LeekHub/leek-fund/blob/master/src/explorer/stockService.ts#L382
    pub async fn get_data(&mut self, stock_code: &str) -> Result<(), Box<dyn std::error::Error>> {
        let res = reqwest::Client::new()
            .get(STOCK_API.to_owned() + stock_code)
            .header("User-Agent","Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.75 Safari/537.36")
            .header("Connection", "Keep-Alive")
            .header("Accept", "text/html, application/xhtml+xml, */*")
            .header("Accept-Language", "en-US,en;q=0.8,zh-Hans-CN;q=0.5,zh-Hans;q=0.3")
            .header("Referer", "https://stock.xueqiu.com/")
            .header("Cookie", self.get_token().await?.as_str())
            .send()
            .await?
            // .json::<ResJson>()
            .text()
            .await?;
        println!("{res:#?}");
        Ok(())
    }
}
