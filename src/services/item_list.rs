use crate::config::data_config::{Item, DATA_CONFIG};
use actix_web::{post, web};
// use serde::Serialize;

#[post("/api/get_item_list")]
async fn get_item_list<'a>() -> web::Json<Vec<&'a Item>> {
    let mut list = Vec::new();

    for (_, item) in DATA_CONFIG.iter() {
        list.push(item)
    }

    web::Json(list)
}
