use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums.clone().into_iter().nth(0);
        let mut min = 0;
        let mut sum = 0;
        
        for num in nums  {
            sum += num;
            max = cmp::max(max, Some(sum - min));
            min = cmp::min(min, sum);
        }
        
        if let Some(number) = max {
            number
        } else {
            0
        }
    }
}

fn main() {
    println!("Hello, world!");
}
