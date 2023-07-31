struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        /* Compute the XOR of all elements in the array */
        let mut xor = 0;

        for num in nums.iter() {
            /* XOR the current element `num` with the previous XOR result */
            xor ^= num;
        }

        /* Find the rightmost set bit of the XOR result */
        let rightmost_set_bit = xor & -xor;

        /* Partition the numbers into two groups based on the rightmost set bit */
        let mut answer = vec![0, 0];

        for num in nums.iter() {
            /* If the current element `num` has the rightmost set bit set */
            if (num & rightmost_set_bit) != 0 {
                answer[0] ^= num;
            } else {
                /* XOR the current element `num` with the second element of `answer` */
                answer[1] ^= num;
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
