// @leet start
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut max: i32 = 0;
        for value in nums {
            if value > max {
                max = value;
            }
            if value > 0 {
                result ^= value;
            }
        }
        for value in 1..max {
            re
        }
    }
}
// @leet end

