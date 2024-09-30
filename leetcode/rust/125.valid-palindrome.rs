impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let len: usize = s.len();
        let s_raw: String = s
            .to_ascii_lowercase()
            .chars()
            .filter(|&character: &char| character.is_alphanumeric())
            .collect();
        s_raw
            .chars()
            .take(len / 2)
            .eq(s_raw.chars().rev().take(len / 2))
    }
}
