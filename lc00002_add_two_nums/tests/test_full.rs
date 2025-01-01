use basic_test_utilities::*;
use lc00002_add_two_nums::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TestCase {
    test_name: String,
    l1: Vec<i32>,
    l2: Vec<i32>,
    expected: Vec<i32>,
}

#[test]
fn test_from_json() {
    let data = std::fs::read_to_string("tests/test_data.json")
        .expect("Failed to read test_data.json file");

    let test_cases: Vec<TestCase> =
        serde_json::from_str(&data).expect("Failed to parse JSON test data");

    for test_case in test_cases {
        println!("Running test: {}", test_case.test_name);

        let l1 = list_to_nodes(&test_case.l1);
        let l2 = list_to_nodes(&test_case.l2);

        let result = nodes_to_list(&add_two_numbers(l1, l2));
        assert_eq!(result, test_case.expected);
    }
}
