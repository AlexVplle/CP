// @leet start
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut start: usize = 0;
        let mut result: usize = usize::MAX;
        let mut sum: i32 = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            if sum >= target {
                while sum - nums[start] >= target {
                    sum -= nums[start];
                    start += 1;
                }
                result = result.min(i - start + 1);
            }
        }
        if result == usize::MAX {
            0
        } else {
            result as i32
        }
    }
}
// @leet end
