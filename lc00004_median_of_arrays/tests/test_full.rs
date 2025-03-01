use lc00004_median_of_arrays::*;
use serde_json::Value;
#[macro_use] extern crate basic_test_utilities;

pub fn test_one(v: &Value) {
    let name = process_params!(v, "test_name", String);
    println!("Running test: {}", name);
    let nums1 = process_params!(v, "nums1", Vec<i32>);
    // let nums1 = serde_json::from_value(v["nums1"].clone()).unwrap();
    let nums2 = serde_json::from_value(v["nums2"].clone()).unwrap();
    let expected = v["expected"].as_f64().unwrap();
    let result = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(result as f64, expected);
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
