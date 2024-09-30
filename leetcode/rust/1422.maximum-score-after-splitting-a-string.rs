impl Solution {
    pub fn max_score(s: String) -> i32 {
        let total_ones = s
            .chars()
            .filter(|&character: &char| character == '1')
            .count() as i32;
        s.chars()
            .take(s.len() - 1)
            .fold(
                (0, 0, total_ones),
                |(max_score, zeroes, ones), character| {
                    let new_zeros: i32 = zeroes + (character == '0') as i32;
                    let new_ones: i32 = ones - (character == '1') as i32;
                    let score: i32 = new_zeros + new_ones;
                    (max_score.max(score), new_zeros, new_ones)
                },
            )
            .0
    }
}
