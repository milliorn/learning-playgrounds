impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        let mut reversed_chars = chars.clone();
        reversed_chars.reverse();
        let reversed = reversed_chars.into_iter().collect::<String>();

        chars.into_iter().collect::<String>() == reversed
    }
}
