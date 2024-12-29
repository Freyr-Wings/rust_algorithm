/// Finds two numbers in `nums` that add up to `target` using a hash map approach.
/// Returns `Some((index1, index2))` if a valid pair is found, otherwise returns `None`.
pub fn two_sum_hash_map(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    use std::collections::HashMap;

    let mut map = HashMap::new(); // number -> index

    for (i, &val) in nums.iter().enumerate() {
        let complement = target - val;
        if let Some(&j) = map.get(&complement) {
            return Some((j, i));
        }
        map.insert(val, i);
    }
    None
}