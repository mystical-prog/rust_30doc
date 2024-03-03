pub fn next_permutation(nums: &mut Vec<i32>) {
    let mut i = nums.len() - 1;
    while i > 0 && nums[i-1] >= nums[i] {
        i -= 1;
    }
    if i != 0 {
        let mut j = nums.len() - 1;
        while nums[j] <= nums[i-1] {
            j -= 1;
        }
        nums.swap(i-1,j);
    }
    nums[i..].reverse()
}