// @leet start
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n: usize = a.len();
        let mut number_seen: Vec<bool> = vec![false; n];
        let mut result: Vec<i32> = vec![0; n];
        let mut number_in_common: i32 = 0;
        for index in 0..n {
            let index_number_seen_a: usize = (a[index] - 1) as usize;
            let index_number_seen_b: usize = (b[index] - 1) as usize;
            number_in_common += if a[index] == b[index] {
                1
            } else {
                number_seen[index_number_seen_a] as i32 + number_seen[index_number_seen_b] as i32
            };
            result[index] = number_in_common;
            number_seen[index_number_seen_a] = true;
            number_seen[index_number_seen_b] = true;
        }
        result
    }
}
// @leet end

