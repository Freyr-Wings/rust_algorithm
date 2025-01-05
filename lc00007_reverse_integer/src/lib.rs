pub fn reverse(x: i32) -> i32 {
    let i32_max_bound = i32::MAX / 10;
    let i32_min_bound = i32::MIN / 10;

    let mut xx = x;
    let mut result = 0i32;
    let mut begin = false;
    while xx != 0 {
        let remain = xx % 10;
        xx /= 10;
        if remain == 0 && !begin {
            continue;
        }
        begin = true;

        if result > i32_max_bound || (result == i32_max_bound && remain > 7) 
            || result < i32_min_bound || (result == i32_min_bound && remain < -8) {
            return 0
        }

        result = result * 10 + remain;
    }

    result
}