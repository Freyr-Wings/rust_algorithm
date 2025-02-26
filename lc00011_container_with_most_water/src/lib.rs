pub fn max_area(height: Vec<i32>) -> i32 {
    let mut result = 0;
    let len = height.len();
    let mut left = 0;
    let mut right = len - 1;

    while left < right {
        if height[left] < height[right] {
            result = i32::max(result, height[left] * (right - left) as i32);
            left += 1;
        }
        else {
            result = i32::max(result, height[right] * (right - left) as i32);
            right -= 1;
        }
    }

    return result;
}
