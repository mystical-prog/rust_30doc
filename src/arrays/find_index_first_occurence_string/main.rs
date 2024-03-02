fn main() {
    let out = str_str("haystack".to_string(), "st".to_string());
    println!("{:?}", out);
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let n = needle.len();
    let h = haystack.len();
    if n > h {
        return -1;
    }
    let max = h - n;
    let mut i = 0;
    println!("{:?}, {:?}", max, n);
    while i <= max {
        if &haystack[i..i+n] == needle {
            return i as i32;
        }
        i += 1;
    }
    -1
}