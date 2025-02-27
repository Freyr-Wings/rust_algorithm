pub fn int_to_roman(num: i32) -> String {
    let char_ten = vec!['I', 'X', 'C', 'M'];
    let char_five = vec!['V', 'L', 'D'];

    let mut num_copy = num;
    let mut base = 0;
    let mut result = String::new();
    while num_copy > 0 {
        let cur_digit = num_copy % 10;
        let mut temp_result = String::new();

        if cur_digit == 9 {
            if base >= char_ten.len()-1 {
                panic!("Value overflow!");
            }
            temp_result.push(char_ten[base]);
            temp_result.push(char_ten[base+1]);
        }
        else if cur_digit == 4 {
            if base > char_five.len()-1 {
                panic!("Value overflow!");
            }
            temp_result.push(char_ten[base]);
            temp_result.push(char_five[base]);
        }
        else {
            if cur_digit >= 5 {
                temp_result.push(char_five[base]);
            }
            let count_for_ten = cur_digit % 5;
            for _ in 0..count_for_ten {
                temp_result.push(char_ten[base]);
            }
        }

        result.insert_str(0, &temp_result);
        num_copy = num_copy / 10;
        base += 1;
    }

    return result;
}