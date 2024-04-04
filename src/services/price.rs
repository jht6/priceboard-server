use crate::common::obj::Res;

use actix_web::{post, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Price {
    current: f32,     // 现价
    percent: f32,     // 涨跌幅
    change: f32,      // 涨跌价格
    currency: String, // 货币
}

#[derive(Serialize, Deserialize, Debug)]
struct StockDto {
    stock_code: String, // 股票代码
}

#[post("/api/get_stock_data")]
async fn get_stock_data(
    data: web::Data<crate::AppState>,
    dto: web::Json<StockDto>,
) -> web::Json<Res<Option<Price>>> {
    let ret = &data.stock_fetcher.get_data(&dto.stock_code).await;
    match ret {
        Ok(x) => web::Json(Res::<Option<Price>> {
            code: 0,
            msg: "".to_string(),
            data: Some(Price {
                current: x.current,
                percent: x.percent,
                change: x.chg,
                currency: x.currency.clone(),
            }),
        }),
        Err(err) => web::Json(Res::<Option<Price>> {
            code: 1,
            msg: err.to_string(),
            data: None,
        }),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinDto {
    coin_name: String, // 币名
}
#[post("/api/get_coin_data")]
async fn get_coin_data(
    data: web::Data<crate::AppState>,
    dto: web::Json<CoinDto>,
) -> web::Json<Res<Option<Price>>> {
    let ret = &data.coin_fetcher.get_data(&dto.coin_name).await;
    match ret {
        Ok(x) => web::Json(Res::<Option<Price>> {
            code: 0,
            msg: "".to_string(),
            data: Some(Price {
                current: x.price,
                percent: x.priceDayChange,
                change: x.priceDayChangeAmount,
                currency: "CNY".to_string(),
            }),
        }),
        Err(err) => web::Json(Res::<Option<Price>> {
            code: 1,
            msg: err.to_string(),
            data: None,
        }),
    }
}
