pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut i = 1;
    let mut last = nums[0];
    while i < nums.len() {
        if nums[i] == last {
            nums.remove(i);
        }
        else {
            last = nums[i];
            i += 1;
        }
    }
    return nums.len() as i32;
}