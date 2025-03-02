use serde_json::Value;
use basic_test_macro::*;

pub fn test_one(v: &Value) {
    extract_data!(v, test_name, String);
    println!("Running test: {}", test_name);
    extract_data!(v, input, String);
    extract_data!(v, expected, String);
    assert_eq!(input, expected);
}

#[test]
fn test_from_json() {
    let data = std::fs::read_to_string("tests/test_data.json")
        .expect("Failed to read test_data.json file");
    let _: Vec<_> = serde_json::from_str::<Value>(&data)
        .expect("Failed to parse JSON test data")
        .as_array()
        .expect("Should be a JSON array")
        .into_iter()
        .map(|v| test_one(v))
        .collect();
}
