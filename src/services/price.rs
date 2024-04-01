use actix_web::{post, web};

#[post("/api/get_stock_data")]
async fn get_stock_data<'a>() -> web::Json<Vec<&'a str>> {
    let list = Vec::new();

    web::Json(list)
}
