struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&i| i != val);
        nums.len() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
