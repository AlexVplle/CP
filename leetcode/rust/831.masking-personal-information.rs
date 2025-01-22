// @leet start
impl Solution {
    pub fn mask_pii(s: String) -> String {
        if s.contains('@') {
            Self::mask_email(&s)
        } else {
            Self::mask_phone(&s)
        }
    }

    pub fn mask_email(s: &str) -> String {
        let s_lower = s.to_lowercase();
        let split_email: Vec<&str> = s_lower.split('@').collect();
        let name = split_email[0];
        let (first, last): (char, char) =
            (name.chars().nth(0).unwrap(), name.chars().last().unwrap());
        format!("{}*****{}@{}", first, last, split_email[1])
    }

    pub fn mask_phone(s: &str) -> String {
        let digits: String = s.chars().filter(|c: &char| c.is_ascii_digit()).collect();
        let local_number: &str = &digits[digits.len() - 4..];
        let country_code_len: usize = digits.len() - 10;
        let country_code_mask: String = if country_code_len > 0 {
            format!("+{}-", "*".repeat(country_code_len))
        } else {
            String::new()
        };
        format!("{}***-***-{}", country_code_mask, local_number)
    }
}
// @leet end
