use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

pub mod common;
pub mod config;
pub mod fetcher;
pub mod services;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!".to_owned())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[post("/api/test")]
async fn test() -> Result<&'static str, Box<dyn std::error::Error>> {
    // let mut sf = fetcher::stock::StockFetcher::new();
    // let s = sf.get_data("00700").await?;
    // println!("s: {:?}", s);
    return Ok("");
}

pub struct AppState {
    stock_fetcher: fetcher::stock::StockFetcher,
    coin_fetcher: fetcher::coin::CoinFetcher,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化fetcher
    let stock_fetcher = fetcher::stock::StockFetcher::new().await;
    let coin_fetcher = fetcher::coin::CoinFetcher::new();

    let app_data = AppState {
        stock_fetcher,
        coin_fetcher,
    };
    let data = web::Data::new(Mutex::new(app_data));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(hello)
            .service(echo)
            .service(test)
            .service(services::item_list::get_item_list)
            .service(services::price::get_stock_data)
            .service(services::price::get_coin_data)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
