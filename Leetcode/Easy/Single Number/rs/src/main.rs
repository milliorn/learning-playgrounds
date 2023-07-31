struct Solution {
    number: i32,
}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut answer = 0;

        for i in &nums {
            answer ^= i;
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
