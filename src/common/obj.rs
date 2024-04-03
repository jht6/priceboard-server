use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Res<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}
