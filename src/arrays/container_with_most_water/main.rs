fn main() {
    let out : i32 = max_area(vec![6,3,8,4,6]);
    println!("{:?}", out);
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut i = 0;
    let mut j = height.len() - 1;
    if j == 1 {
        let _min = min(height[0], height[1]);
        return _min * _min;
    }
    while j > i {
        let _min = min(height[i], height[j]) * ((j-i) as i32);
        if _min > max {
            max = _min;
        }
        if height[i] >= height[j] {
            j -= 1;
        } else {
            i += 1;
        }
    }
    max
}

pub fn min(n1: i32, n2: i32) -> i32 {
    if n1 >= n2 {
        return n2;
    } else {
        return n1;
    }
}