pub fn length_of_longest_substring(s: String) -> i32 {
    let str_len = s.len() as i32;
    let mut left = 0 as i32;
    let mut right = 0 as i32;
    let mut max_len = 0;

    while right < str_len && left <= right {
        if left == right || !s[left as usize..(right) as usize].contains(&s[right as usize..(right+1) as usize]) {
            right += 1;
        } else {
            left += 1;
            continue;
        }
        let cur_len = right - left;
        max_len = if cur_len > max_len { cur_len } else { max_len };
    }

    max_len
}