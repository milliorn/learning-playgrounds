struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map_or(-1 as i32, |v| v as i32)
    }
}

fn main() {
    println!("Hello, world!");
}
