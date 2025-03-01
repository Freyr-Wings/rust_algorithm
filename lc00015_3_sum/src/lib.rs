use std::collections::HashMap;


pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results: Vec<Vec<i32>> = vec![];
    let mut records = HashMap::new();

    for v in nums {
        if !records.contains_key(&v) {
            records.insert(v, 0);
        }
        records.insert(v, records[&v] + 1);
    }

    let key_three = 0;
    if records.contains_key(&key_three) && records[&key_three] >= 3 {
        results.push(vec![0, 0, 0]);
    }

    let mut keys:Vec<i32> =  records.keys().cloned().collect();
    keys.sort();
    let len_keys = keys.len();

    for i in 0..len_keys {
        if records[&keys[i]] < 2 {
            continue;
        }
        let k_match = -2*keys[i];
        if k_match != keys[i] && records.contains_key(&k_match) {
            if k_match < keys[i] {
                results.push(vec![k_match,keys[i],keys[i]]);
            }
            else {
                results.push(vec![keys[i],keys[i],k_match]);
            }
            
        }
    }

    for i in 0..len_keys {
        for j in i+1..len_keys {
            let k_match = -keys[i]-keys[j];
            if k_match == keys[i] || k_match == keys[j] 
                || !records.contains_key(&k_match)
                || k_match < keys[j] {
                continue;
            }
            results.push(vec![keys[i], keys[j], k_match]);
        }
    }

    return results;
}
