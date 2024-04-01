use crate::common::consts::data_type::{DATA_TYPE_COIN, DATA_TYPE_STOCK};
use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub data_type: String,
}

lazy_static! {
    #[derive(Debug)]
    pub static ref DATA_CONFIG: HashMap<String, Item> = {
        let mut map = HashMap::new();

        map.insert(
            "btc".to_string(),
            Item {
                id: "btc".to_string(),
                name: "比特币".to_string(),
                data_type: DATA_TYPE_COIN.to_string(),
            },
        );

        map.insert(
            "tecent".to_string(),
            Item {
                id: "tecent".to_string(),
                name: "腾讯".to_string(),
                data_type: DATA_TYPE_STOCK.to_string(),
            },
        );

        map
    };
}
