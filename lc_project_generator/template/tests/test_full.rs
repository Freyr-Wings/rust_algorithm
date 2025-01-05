use serde_json::Value;

pub fn test_one(v: &Value) {
    let name = v["test_name"].as_str().unwrap();
    println!("Running test: {}", name);
    let input = v["input"].as_str().unwrap();
    let expected = v["expected"].as_str().unwrap();
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
