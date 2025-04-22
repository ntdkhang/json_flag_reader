use std::env;
use serde_json::Result;
use serde::{Serialize, Deserialize};
use std::fs;

fn main() {
    let file_path = parse_args();
    let json_string = match fs::read_to_string(&file_path) {
       Err(e) => {
           eprintln!("Unable to read from file path {}. Error is: {}", &file_path, e);
           std::process::exit(1);
       },
       Ok(v) => v
    };
    let feature_flag: FeatureFlag = match serde_json::from_str(&json_string) {
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Unable to parse json from string");
            std::process::exit(1);
        },
        Ok(v) => v
    };
    println!("Feature Flag: {:?}", feature_flag);
}


fn parse_args() -> String {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Expected 1 argument, got {} instead", args.len());
        std::process::exit(1);
    }
    args[0].clone()
}

#[derive(Serialize, Deserialize, Debug)]
struct FeatureFlag {
    name: String,
    latitude: f64,
    longitude: f64,
    visit: bool
}
