// @leet start
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut result: i32 = s.len() as i32;
        let mut count_array: [usize; 26] = [0; 26];
        for c in s.chars() {
            let index: usize = c as usize - 'a' as usize;
            count_array[index] += 1;
            if count_array[index] > 2 {
                count_array[index] -= 2;
                result -= 2;
            }
        }
        result
    }
}
// @leet end

