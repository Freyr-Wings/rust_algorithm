pub fn convert(s: String, num_rows: i32) -> String {
    let len = s.len();
    if len == 0 || num_rows == 1 {
        return s;
    }
    let mut records = vec![String::new();num_rows as usize];
    let mut down = -1i32;
    let mut row = 0i32;
    for c in s.chars() {
        records[row as usize].push(c);
        if ((row as i32 + 1) % num_rows == 0) || row == 0{
            down = -down;
        }
        row += down;
    }

    return records.join("");
}