use std::env;
use serde::{Serialize, Deserialize};
use std::fs;

fn main() {
    let file_path = parse_arg_path();
    let json_string = read_file(&file_path);
    let feature_flag = read_flag(&json_string);
    println!("Feature Flag: {:?}", feature_flag);
}

fn read_file(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Err(e) => {
            eprintln!("Unable to read from file path {}. Error is: {}", &file_path, e);
            std::process::exit(1);
        },
        Ok(v) => v
    }
}

fn read_flag(from_str: &str) -> FeatureFlag {
    match serde_json::from_str(from_str) {
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Unable to parse json from string");
            std::process::exit(1);
        },
        Ok(v) => v
    }
}

fn parse_arg_path() -> String {
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

#[test]
fn test_read_flag() {
    let json_string = r#"
{
    "name": "Damri",
    "latitude": 11.5380222,
    "longitude": 107.6474407,
    "visit": true
}
    "#;
    let feature_flag = read_flag(&json_string);
    assert_eq!(feature_flag.name, "Damri");
    assert_eq!(feature_flag.latitude, 11.5380222);
    assert_eq!(feature_flag.longitude, 107.6474407);
    assert_eq!(feature_flag.visit, true);
}
