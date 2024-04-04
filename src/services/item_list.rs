use crate::common::obj::Res;
use crate::config::data_config::{Item, DATA_CONFIG_LIST};
use actix_web::{post, web};
// use serde::Serialize;

#[post("/api/get_item_list")]
async fn get_item_list() -> web::Json<Res<Vec<&'static Item>>> {
    let mut list = Vec::new();

    for item in DATA_CONFIG_LIST.iter() {
        list.push(item)
    }

    web::Json(Res::<Vec<&Item>> {
        code: 0,
        msg: "ok".to_string(),
        data: list,
    })
}
