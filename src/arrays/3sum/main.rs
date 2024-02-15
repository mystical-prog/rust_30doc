use std::collections::HashSet;

fn main() {
    let result = three_sum(vec![1,-1,3,-4,5,2,1,3,-2,-3]);
    println!("{:?}", result);
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut temp : Vec<i32> = nums.clone();
    temp.sort();
    let n : usize = nums.len();
    let mut output : HashSet<Vec<i32>> = HashSet::new();
    for i in 0..n-2{
        let mut j : usize = i + 1;
        let mut k : usize = n - 1;

        while k > j {
            let sum = temp[i] + temp[j] + temp[k];
            if sum == 0 {
                output.insert(vec![temp[i], temp[j], temp[k]]);
                let _n2 = temp[j];
                while k > j && temp[j] == _n2 {
                    j += 1;
                }
                let _n3 = temp[k];
                while k > j && temp[k] == _n3 {
                    k -= 1;
                }
            } else if sum < 0 {
                j += 1;
            } else {
                k -= 1;
            }
        }
    }
    output.into_iter().collect::<Vec<Vec<i32>>>()
}