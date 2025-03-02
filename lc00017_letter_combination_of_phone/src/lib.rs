use std::collections::{HashMap, VecDeque};

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }

    let phone_map: HashMap<char, &str> = HashMap::from([
        ('2', "abc"), ('3', "def"), ('4', "ghi"), ('5', "jkl"),
        ('6', "mno"), ('7', "pqrs"), ('8', "tuv"), ('9', "wxyz")
    ]);

    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(String::new());

    for digit in digits.chars() {
        if let Some(&letters) = phone_map.get(&digit) {
            let mut temp_queue = VecDeque::new();
            while let Some(combination) = queue.pop_front() {
                for letter in letters.chars() {
                    let new_combination = format!("{}{}", combination, letter);
                    temp_queue.push_back(new_combination);
                }
            }
            queue = temp_queue;
        }
        else {
            panic!("Invalid number");
        }
    }

    queue.into_iter().collect()
}

