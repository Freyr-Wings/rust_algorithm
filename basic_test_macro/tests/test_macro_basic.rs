use basic_test_macro::{extract_data, type_converter};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_invalid_hashmap_generic_params() {
    let data = json!({"key": {"inner": 1}});
    extract_data!(data, key, HashMap<String, i32>);
    assert!(key.contains_key("inner"));
    assert_eq!(1, key["inner"]);
}
