fn main() {
	let result = two_sum(vec![1,2,3,4], 5);
	println!("{:?}" , result);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
	let n : usize = nums.len();
	if n == 2 {
		return vec![0,1];
	}
	for i in 0..n {
		for j in (i+1)..n {
			if nums[i] + nums[j] == target {
				return vec![i as i32,j as i32];
			}
		}
	}
	vec![0,0]
}