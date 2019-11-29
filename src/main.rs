extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub channel_tag: i32,
    pub alternatives: Vec<Alternatives>,
    pub language_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Alternatives {
    pub confidence: f32,
    pub transcript: String,
    pub words: Vec<Words>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Words {
    pub end_time: String,
    pub word: String,
    pub start_time: String,
}


fn main() {
    println!("Hello, world!");
}
