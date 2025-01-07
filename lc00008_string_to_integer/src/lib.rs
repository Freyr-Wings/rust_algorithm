pub fn my_atoi(s: String) -> i32 {
    let chars = s.trim().chars().collect::<Vec<char>>();
    let mut result = 0i32;

    let slen = chars.len();

    let mut state = 0;
    let mut positive = true;

    if slen == 0 {
        return result;
    }

    for i in 0..slen {
        let cur = chars[i];
        if state == 0 {
            if cur == '+' {
                positive = true;
                state = 1;
            }
            else if cur == '-' {
                positive = false;
                state = 1;
            }
            else if is_digit(cur) {
                state = 1;
                if handle_digit(&mut result, cur, positive) {
                    break;
                }
            }
            else {
                break;
            }
        }
        else if state == 1 {
            if is_digit(cur) {
                if handle_digit(&mut result, cur, positive) {
                    break;
                }
            }
            else {
                break;
            }
        }
        else {
            break;
        }
    }

    return result;
}

fn is_digit(c: char) -> bool {
    let pos = get_digit(c);
    return pos >= 0 && pos < 10;
}

fn get_digit(c: char) -> i32 {
    return c as i32 - '0' as i32; 
}

fn add_digit(cur: i32, digit: i32) -> i32 {
    if cur > i32::MAX / 10 || 
        cur == i32::MAX / 10 && digit >= 7 {
        return i32::MAX;
    }
    else if cur < i32::MIN / 10 ||
        cur == i32::MIN / 10 && digit <= -8 {
        return i32::MIN;
    }
    return cur * 10 + digit;
}

fn handle_digit(cur: &mut i32, c: char, positive: bool) -> bool {
    let mut digit = get_digit(c);
    if !positive {
        digit = -digit;
    }
    *cur = add_digit(*cur, digit);
    if *cur == i32::MAX || *cur == i32::MIN {
        return true;
    }
    return false;
}