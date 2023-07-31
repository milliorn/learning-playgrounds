struct Solution {}

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        use std::i64;
        let min: i64 = i64::MIN;
        let mut max1 = min;
        let mut max2 = min;
        let mut max3 = min;

        for element in nums.iter() {
            let element = *element as i64;
            if element == max1 || element == max2 || element == max3 {
                continue;
            }

            if element > max1 {
                max3 = max2;
                max2 = max1;
                max1 = element;
            } else if element > max2 {
                max3 = max2;
                max2 = element;
            } else if element > max3 {
                max3 = element;
            }
        }

        return if max3 == min {
            max1 as i32
        } else {
            max3 as i32
        };
    }
}

fn main() {
    println!("Hello, world!");
}
