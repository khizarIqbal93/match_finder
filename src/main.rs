use std::env;
use std::fs;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Person {
 id: String,
 name: String,
 love_score: u8
}

fn parse_json(json_string: String) -> Vec<Person> {
    let mut data_set: Vec<Person> = Vec::new();
    match serde_json::from_str(&json_string.to_string()) {
        Ok(valid_json) => {
            data_set = valid_json;
        } Err(e) => {
            println!("{e}")
        }
    }
    data_set
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file_content= fs::read_to_string(file_path).expect("failed trying to read file");
    let data = parse_json(file_content);

    let mut i: usize= 0;
    let length = data.len();

    while i < length {
        let element = &data[i];
        println!("{element:#?}");
        i += 1;
    }
}
