use lc00018_4_sum::four_sum;
use serde_json::Value;
use std::fmt::Debug;
use basic_test_macro::*;


pub fn test_one(v: &Value) {
    extract_data!(v, test_name, String);
    println!("Running test: {}", test_name);
    extract_data!(v, nums, Vec<i32>);
    extract_data!(v, target, i32);
    extract_data!(v, expected, Vec<Vec<i32>>);
    let result = four_sum(nums, target);
    assert!(are_vectors_equal(result, expected));
}

fn are_vectors_equal<T: Ord + Clone + Debug>(mut vec1: Vec<Vec<T>>, mut vec2: Vec<Vec<T>>) -> bool {
    for inner in &mut vec1 {
        inner.sort();
    }
    for inner in &mut vec2 {
        inner.sort();
    }

    vec1.sort();
    vec2.sort();
    println!("{:?}", vec1);
    println!("{:?}", vec2);

    vec1 == vec2
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
