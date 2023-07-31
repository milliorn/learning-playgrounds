#[allow(unused)]
struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut temp = 0;

        (1..nums.len()).for_each(|i| {
            if nums[i] != nums[temp] {
                temp += 1;
                nums[temp] = nums[i]
            }
        });

        (temp + 1) as i32
    }
}

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(1);
    vec.push(2);
    let temp = Solution::remove_duplicates(&mut vec);
    println!("Hello, world!");
    println!("temp = {}", temp);
}
