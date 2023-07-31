struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut xx: i32 = x;
        let mut result: i32 = 0;
        let mut temp: i32 = 0;
        while xx != 0 {
            temp = result * 10 + xx % 10;
            if temp / 10 != result {
                return 0
            }
            result = temp;
            xx /= 10
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
