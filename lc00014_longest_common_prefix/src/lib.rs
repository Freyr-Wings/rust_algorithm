pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let len = strs.len();
    if len == 0 {
        return String::new();
    }

    let mut max_len = strs[0].len();
    
    for i in 1..len {
        max_len = get_common_prefix_len(&strs[0], &strs[i], max_len);
        if max_len == 0 {
            return String::new();
        }
    }

    return strs[0].chars().take(max_len).collect();
}

pub fn get_common_prefix_len(a: &str, b: &str, max_len: usize) -> usize {
    return a.chars()
        .zip(b.chars())
        .take(max_len)
        .take_while(|(c1, c2)| c1 == c2)
        .count();
}