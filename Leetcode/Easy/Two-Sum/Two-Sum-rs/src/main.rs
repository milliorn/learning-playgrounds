fn main() {}

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for (i, _) in nums.iter().enumerate() {
            for (j, _) in nums.iter().enumerate() {
                let a = i as i32;
                let b = j as i32;
                if a == b {
                    continue;
                }
                if (nums[i] + nums[j]) == target {
                    result.push(a);
                    result.push(b);
                    return result;
                }
            }
        }
        result
    }
}
