struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let str = s.trim();
        if str.chars().count() < 2 || !str.contains(char::is_whitespace) {
            return str.chars().count() as i32;
        }
        let (_, x) = s.trim_end().rsplit_once(' ').unwrap();
        x.chars().count() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
