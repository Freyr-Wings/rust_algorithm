pub fn longest_palindrome(s: String) -> String {
    let str_len = s.len();
    if str_len == 0 {
        return s;
    }
    
    let mut records = vec![0; 2*str_len-1];
    records[0] = 0;

    let mut max_idx_start = 0;
    let mut max_len = 1;
    let mut cur_max_center = 0;
    let mut cur_max_end = 0;

    for i in 1..2*str_len-1 {
        let mut cur_span_len = 0;
        
        let cur_end = (i as i32)/2 + (cur_span_len as i32);
        if  cur_max_end > cur_end {
            let symmetric_len: usize = records[(cur_max_center-(i as i32-cur_max_center)) as usize];
            if symmetric_len > cur_span_len {
                cur_span_len = i32::min(symmetric_len as i32, cur_max_end - (i as i32)/2) as usize;
            }
        }

        let mut left = (i+1) as i32/2-(cur_span_len as i32) - 1;
        let mut right = (i) as i32/2+(cur_span_len as i32)+ 1;
        while left >= 0 && right < str_len as i32 
            && s[left as usize..(left+1) as usize] == s[right as usize..(right+1) as usize]{
            cur_span_len += 1;
            left -= 1;
            right += 1;
        }

        records[i] = cur_span_len;
        if right - 1 > cur_max_end {
            cur_max_center = i as i32;
            cur_max_end = right - 1;
            let cur_full_len = cur_span_len * 2 + if i % 2 == 0 {1} else {0};
            if cur_full_len > max_len {
                max_len = cur_full_len;
                max_idx_start = left + 1;
            }
        }
    }

    s[max_idx_start as usize..max_idx_start as usize+max_len].to_string()
}
