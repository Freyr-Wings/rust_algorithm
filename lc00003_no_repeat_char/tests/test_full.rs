use lc00003_no_repeat_char::*;
use serde_json::Value;

pub fn test_one(v: &Value) {
    dbg!(v);
    let name = v["test_name"].as_str().unwrap();
    println!("Running test: {}", name);
    let s = v["s"].as_str().unwrap();
    let expected = v["expected"].as_i64().unwrap();
    let result = length_of_longest_substring(s.to_string());
    assert_eq!(result as i64, expected);
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
