pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    if len < 3 {
        panic!("Too few nums");
    }

    let mut result = 0;
    let mut min_gap = i32::MAX;
    nums.sort();

    for i in 0..len {
        let mut left = i + 1;
        let mut right = len - 1;
        while left < right {
            let cur = nums[i] + nums[left] + nums[right];
            let diff = i32::abs(cur - target);
            if diff < min_gap {
                min_gap = diff;
                result = cur;
            }
            if cur == target {
                return target;
            }
            else if cur < target {
                left += 1;
                while left < right && nums[left] == nums[left-1] {
                    left += 1;
                }
            }
            else {
                right -= 1;
                while left < right && nums[right] == nums[right+1] {
                    right -= 1;
                }
            }
        }
    }

    return result;
}