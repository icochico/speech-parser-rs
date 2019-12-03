extern crate serde;
extern crate serde_json;
extern crate indicatif;
extern crate rayon;


use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use indicatif::ProgressIterator;
use serde_json::Value;
use std::prelude::v1::Vec;

const TRANSCRIPTS_DIR: &str = "../../raw_transcripts";

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
    let paths = fs::read_dir(TRANSCRIPTS_DIR).unwrap();
    let mut transcripts: Vec<String> = Vec::new();

    for path in paths.progress() {
        let transcript = fs::read_to_string(path.unwrap().path()).unwrap();
//        println!("{}", transcript);
        transcripts.push(transcript);

        // Parse the string of data into serde_json::Value.
//        let v: Value = serde_json::from_str(data)?;

//        println!("{}", path.unwrap().path().display())
    }

    println!("Loaded {} transcripts!", transcripts.len())
}
