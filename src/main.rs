use json_flag_reader::*;

fn main() {
    let feature_flag: FeatureFlag = parse_flag_from_arg();
    println!("FeatureFlag is: {:?}", feature_flag);
}


