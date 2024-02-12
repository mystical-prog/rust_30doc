fn main() {
	let result = find_median_sorted_arrays(vec![1,3], vec![2,4]);
	println!("{:?}", result);
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total_len : usize = nums1.len() + nums2.len();
    let len_1 : usize = nums1.len();
    let len_2 : usize = nums2.len();
    if len_1 == 0 || len_2 == 0 {
        if len_1 == 0 {
            if len_2 % 2 == 0 {
                return (nums2[len_2 / 2] + nums2[(len_2 / 2) - 1]) as f64 / 2.0;
            } else {
                return nums2[(len_2 / 2) + 1] as f64;
            }
        } else {
            if len_1 % 2 == 0 {
                return (nums1[len_1 / 2] + nums1[(len_1 / 2) - 1]) as f64 / 2.0;
            } else {
                return nums1[(len_1 / 2) + 1] as f64;
            }            
        }
    }
    let mut iter_1 : usize = 0;
    let mut iter_2 : usize = 0;
    let mut count : usize = 0;
    if total_len % 2 == 0 {
        let mut prev_val : i32 = 0;
        let mut curr_val : i32 = 0;
        while count < (total_len / 2) + 1 {
            prev_val = curr_val;
            if (nums1[iter_1] <= nums2[iter_2] && iter_1 < len_1 - 1) || iter_2 == len_2 - 1 {
                curr_val = nums1[iter_1];
                iter_1 += 1;
            } else if (nums1[iter_1] > nums2[iter_2] && iter_2 < len_2 - 1) || iter_1 == len_1 - 1 {
                curr_val = nums2[iter_2];
                iter_2 += 1;
            }
            count += 1;
            println!("{}, {}, {}", iter_1, iter_2, curr_val);
        }
        return ((prev_val + curr_val) as f64) / 2.0
    } else {
        let mut curr_val : i32 = 0;
        while count < (total_len / 2) + 1 {
            if (nums1[iter_1] <= nums2[iter_2] && iter_1 < len_1) || iter_2 == len_2 {
                curr_val = nums1[iter_1];
                iter_1 += 1;
            } else if (nums1[iter_1] > nums2[iter_2] && iter_2 < len_2) || iter_1 == len_1 {
                curr_val = nums2[iter_2];
                iter_2 += 1;
            }
            println!("{}, {}, {}", iter_1, iter_2, curr_val);
            count += 1;
        }
        return curr_val as f64;
    }
}
