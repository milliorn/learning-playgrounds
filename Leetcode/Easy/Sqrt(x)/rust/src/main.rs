struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }

        let mut left = 1;
        let mut right = x;
        let mut res = 1;

        while left <= right {
            let mid = (right - left) / 2 + left;
            if x / mid == mid {
                return mid;
            };
            if mid > x / mid {
                right = mid - 1;
            } else {
                left = mid + 1;
                res = mid;
            }
        }
        res
    }
}

fn main() {}
