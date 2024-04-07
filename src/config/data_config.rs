use crate::common::consts::data_type::{DATA_TYPE_COIN, DATA_TYPE_STOCK};
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Serialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub data_type: String,
}

lazy_static! {
    #[derive(Debug,Clone,Serialize)]
    pub static ref DATA_CONFIG_LIST: Vec<Item> = {
        let list = Vec::from([
            Item {
                id: "BTC".to_string(),
                name: "BTC".to_string(),
                data_type: DATA_TYPE_COIN.to_string(),
            },
            Item {
                id: "ETH".to_string(),
                name: "ETH".to_string(),
                data_type: DATA_TYPE_COIN.to_string(),
            },
            Item {
                id: "BNB".to_string(),
                name: "BNB".to_string(),
                data_type: DATA_TYPE_COIN.to_string(),
            },
            Item {
                id: "DOGE".to_string(),
                name: "DOGE".to_string(),
                data_type: DATA_TYPE_COIN.to_string(),
            },
            Item {
                id: "SHIB".to_string(),
                name: "SHIB".to_string(),
                data_type: DATA_TYPE_COIN.to_string(),
            },
            Item {
                id: "UNI".to_string(),
                name: "UNI".to_string(),
                data_type: DATA_TYPE_COIN.to_string(),
            },
            Item {
                id: "00700".to_string(),
                name: "腾讯".to_string(),
                data_type: DATA_TYPE_STOCK.to_string(),
            },
        ]);

        list
    };
}
