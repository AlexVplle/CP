// @leet start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen: [Option<usize>; 128] = [None; 128];
        let mut max_length: usize = 0;
        let mut start: usize = 0;
        for (index, char) in s.char_indices() {
            let byte = char as usize;
            if let Some(prev_index) = last_seen[byte] {
                if prev_index >= start {
                    start = prev_index + 1;
                }
            }
            last_seen[byte] = Some(index);
            max_length = max_length.max(index - start + 1);
        }
        max_length as i32
    }
}
// @leet end
