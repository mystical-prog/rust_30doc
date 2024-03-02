pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    let mut count = nums.len();
    while i < nums.len() {
        if nums[i] == val {
            nums.remove(i);
            count -= 1;
        } else {
            i += 1;
        }
    }
    count as i32
}