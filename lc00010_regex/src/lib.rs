
pub fn is_match(s: String, p: String) -> bool {
    let s_len = s.len();
    let p_len = p.len();
    
    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = p.chars().collect();

    let mut records = vec![vec![false; s_len+1]; p_len+1];
    records[0][0] = true;

    for i in 2..=p_len {
        if p_chars[i-1] == '*' {
            records[i][0] = records[i-2][0];
        }
    }

    for i in 1..=p_len {
        for j in 1..=s_len {
            if p_chars[i-1] == '.' {
                records[i][j] = records[i-1][j-1];
            }
            else if p_chars[i-1] == '*' {
                if i == 1 || p_chars[i-2] == '*' {
                    panic!("Unexpected location for '*'");
                } 
                if s_chars[j-1] == p_chars[i-2] || p_chars[i-2] == '.' {
                    records[i][j] = records[i-1][j-1] || records[i][j-1];
                }
                records[i][j] |= records[i-2][j];
            }
            else if s_chars[j-1] == p_chars[i-1] && records[i-1][j-1] {
                records[i][j] = true;
            }
            else {
                records[i][j] = false;
            }
        }
    }

    return records[p_len][s_len];
}