use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        return {
            let numerals = get_roman_numerals();
            let mut x: i32 = numerals[&s.chars().last().unwrap()];
            for i in 0..(s.len() - 1) {
                let temp = s.chars().nth(i).unwrap();
                if numerals[&temp] >= numerals[&s.chars().nth(i + 1).unwrap()] {
                    x += numerals[&temp]
                } else {
                    x -= numerals[&temp];
                }
            }
            x
        };
    }
}

fn get_roman_numerals() -> HashMap<char, i32> {
    let mut numerals: HashMap<char, i32> = HashMap::new();
    numerals.insert('I', 1);
    numerals.insert('V', 5);
    numerals.insert('X', 10);
    numerals.insert('L', 50);
    numerals.insert('C', 100);
    numerals.insert('D', 500);
    numerals.insert('M', 1000);
    numerals
}

fn main() {
    let three = Solution::roman_to_int("III".to_string());
    let five_eight = Solution::roman_to_int("LVIII".to_string());
    let one_nine_nine_four = Solution::roman_to_int("MCMXCIV".to_string());

    println!("{}", three);
    println!("{}", five_eight);
    println!("{}", one_nine_nine_four);
}
