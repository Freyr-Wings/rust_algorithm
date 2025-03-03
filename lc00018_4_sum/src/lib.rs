use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;


pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort();

    let mut results: Vec<Vec<i32>> = vec![];

    let mut records_one_sum: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        records_one_sum.entry(nums[i]).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut records_two_sum: HashMap<i64, HashMap<i32, i32>> = HashMap::new();
    for key in records_one_sum.keys() {
        for key_another in records_one_sum.keys() {
            if key == key_another && records_one_sum[key] == 1 { 
                continue;
            }
            if key > key_another {
                continue;
            }
            let sum = (*key as i64) + (*key_another as i64);
            if !records_two_sum.contains_key(&sum) {
                records_two_sum.insert(sum, HashMap::new());
            }
            let records_two_sum_inner = records_two_sum.get_mut(&sum).unwrap();
            if !records_two_sum_inner.contains_key(&key) {
                records_two_sum_inner.insert(*key, *key_another);
            }
            records_two_sum_inner.entry(*key).or_insert(*key_another);
        }
    }

    let mut records_four_sum: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    for key in records_two_sum.keys() {
        let res = target as i64 - *key;
        if !records_two_sum.contains_key(&res) {
            continue;
        }
        let comb_a = records_two_sum.get(key).unwrap();
        let comb_b = records_two_sum.get(&res).unwrap();

        for key_a in comb_a.keys() {
            for key_b in comb_b.keys() {
                let mut quad = vec![
                    *key_a, *comb_a.get(key_a).unwrap(),
                    *key_b, *comb_b.get(key_b).unwrap()
                ];
                if validate_quad(&records_one_sum, &quad) {
                    quad.sort();
                    if records_four_sum.insert((quad[0], quad[1], quad[2], quad[3])) {
                        results.push(quad.clone());
                    }
                }
            }
        }
    }

    return results;
}

fn validate_quad(records: &HashMap<i32, i32>, quad: &Vec<i32>) -> bool {
    let mut quad_records = HashMap::new();
    for num in quad {
        quad_records.entry(num).and_modify(|c| *c += 1).or_insert(1);
    }
    for k in quad_records.keys() {
        if quad_records.get(k).unwrap() > records.get(k).unwrap() {
            return false;
        }
    }
    return true;
}

pub fn four_sum_v1(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let n = nums.len();
    if n < 4 {
        return vec![]
    }
    nums.sort_unstable();
    let mut res = vec![];
    for (i, &a) in nums[..n-3].iter().enumerate() {
        if i > 0 && nums[i-1] == a || (a as i64 + nums[n-1] as i64 + nums[n-2] as i64) < (target as i64 - nums[n-3] as i64) {
            continue
        } else if (a as i64 + nums[i+1] as i64 + nums[i+2] as i64) > (target as i64 - nums[i+3] as i64) {
            break
        }
        let mut three_sum = three_sum(&nums[i+1..], target - a);
        for team in &mut three_sum {
            team.push(a);
        }
        res.append(&mut three_sum);
    }
    res
}

fn three_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let n = nums.len();
    for (i, &a) in nums[0..n-2].iter().enumerate() {
        if i > 0 && a == nums[i - 1] || a + nums[n - 1] < target - nums[n-2] {
            continue
        }
        if a + nums[i + 1] > target - nums[i+2] {
            break
        }
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            match (a + nums[j] + nums[k]).cmp(&target) {
                Ordering::Greater => k -= 1,
                Ordering::Equal => {
                    res.push(vec![a, nums[j], nums[k]]);
                    j += 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1
                    }
                    k -= 1;
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1
                    }
                },
                Ordering::Less => j += 1,
            }
        }
    }
    res
}