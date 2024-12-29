use serde::Deserialize;
use lc00001_two_sum::two_sum_hash_map;

#[derive(Deserialize, Debug)]
struct TestCase {
    test_name: String,
    nums: Vec<i32>,
    target: i32,
    /// Use `Option<[usize; 2]>` to represent either two indices or None if no pair expected
    expected: Option<[usize; 2]>,
}

#[test]
fn test_from_json() {
    // 1. Load the JSON file from the tests directory
    let data = std::fs::read_to_string("tests/test_data.json")
        .expect("Failed to read test_data.json file");

    // 2. Deserialize the contents into a Vec<TestCase>
    let test_cases: Vec<TestCase> =
        serde_json::from_str(&data).expect("Failed to parse JSON test data");

    // 3. Run each test case
    for test_case in test_cases {
        println!("Running test: {}", test_case.test_name);

        // Execute both solutions
        let result_hash = two_sum_hash_map(test_case.nums, test_case.target);

        match test_case.expected {
            Some(expected_pair) => {
                assert_eq!(
                    result_hash,
                    Some((expected_pair[0], expected_pair[1])),
                    "Hash map failed for {}",
                    test_case.test_name
                );
            }
            None => {
                assert_eq!(
                    result_hash,
                    None,
                    "Hash map should return None for {}",
                    test_case.test_name
                );
            }
        }
    }
}
