impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let str: String = x.to_string();
        let rev: String = str.chars().rev().collect::<String>();
        str == rev
    }
}
