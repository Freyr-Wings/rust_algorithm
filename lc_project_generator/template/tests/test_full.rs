use serde_json::Value;
#[macro_use] extern crate basic_test_utilities;

pub fn test_one(v: &Value) {
    let name = process_params!(v, "test_name", String);
    println!("Running test: {}", name);
    let input = process_params!(v, "input", String);
    let expected = process_params!(v, "expected", String);
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
