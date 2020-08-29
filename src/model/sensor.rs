use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddSensor {
    pub words: Vec<String>,
    pub language: String,
}