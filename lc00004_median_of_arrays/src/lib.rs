pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let len1 = nums1.len();
    let len2 = nums2.len();
    if len2 > len1 {
        return find_median_sorted_arrays(nums2, nums1);
    }
    if len1 == 0 {
        return 0.0;
    }

    let mut left = 0;
    let mut right = 2*len2;
    while left <= right {
        let mid2 = (left + right) / 2;
        let mid1 = len1 + len2 - mid2;
        
        let l1 = if mid1 == 0 {i32::MIN} else {nums1[(mid1-1) / 2]};
        let r1 = if mid1 == len1*2 {i32::MAX} else {nums1[mid1 / 2]};
        let l2 = if mid2 == 0 {i32::MIN} else {nums2[(mid2-1) / 2]};
        let r2 = if mid2 == len2*2 {i32::MAX} else {nums2[mid2 / 2]};

        if l1 > r2 {
            left = mid2 + 1;
        }
        else if l2 > r1 {
            right = mid2 - 1;
        }
        else {
            return ((i32::max(l1, l2) as f64) + (i32::min(r1, r2) as f64)) / 2.0;
        }
    }

    0.0
}

