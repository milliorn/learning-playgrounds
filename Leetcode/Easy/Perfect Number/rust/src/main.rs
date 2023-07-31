struct Solution {}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num <= 1 {
            return false;
        }

        let mut sum = 1;
        let mut divisor = 2;

        while (divisor * divisor <= num) {
            if num % divisor == 0 {
                sum += divisor;
                if divisor != num / divisor {
                    sum += num / divisor;
                }
            }
            divisor += 1;
        }
        sum == num
    }
}

fn main() {
    println!("Hello, world!");
}
