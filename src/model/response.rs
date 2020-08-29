use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSuccess {
    pub code: i32,
    pub data: serde_json::Value,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
}
