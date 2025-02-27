use std::collections::HashMap;

pub fn roman_to_int(s:String) -> i32 {
    let roman_map = HashMap::from([
        ('I',1),
        ('V',5),
        ('X',10),
        ('L',50),
        ('C',100),
        ('D',500),
        ('M',1000)
    ]);

    let mut total = 0;
    let mut prev_value = 0;

    for c in s.chars().rev() {
        if let Some(&value) = roman_map.get(&c) {
            if value < prev_value {
                total -= value;
            } else {
                total += value;
            }
            prev_value = value;
        } else {
            panic!("Invalid Roman numeral character: {}", c);
        }
    }

    return total;
}
