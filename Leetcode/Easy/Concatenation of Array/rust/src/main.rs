pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut a = nums.to_vec();
    let b = nums.to_vec();

    a.extend(b);
    return a;
}

fn main() {
    println!("Hello, world!");
}
