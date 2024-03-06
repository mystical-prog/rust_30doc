pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut i = 0;
    let mut j = nums.len() - 1;
    if j == 0 {
        if nums[0] == target {
            return 0;
        } else {
            return -1;
        }
    }
    while j > i {
        if nums[i] == target {
            return i as i32;
        } else if nums[j] == target {
            return j as i32;
        } else {
            if nums[i] > target {
                j -= 1;
            } else {
                i += 1;
            }
        }
    }
    -1
}