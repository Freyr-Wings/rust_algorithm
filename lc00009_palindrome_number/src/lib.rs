pub fn is_palindrome(x: i32) -> bool {
    if x == 0 {
        return true;
    }
    if x < 0 || x % 10 == 0{
        return false;
    }

    let mut record = 0;
    let mut xcopy = x;

    while xcopy > record {
        record = record * 10 + xcopy % 10;
        if record == xcopy {
            return true;
        }
        xcopy /= 10;
        if record == xcopy {
            return true;
        }
    }

    return false;
}