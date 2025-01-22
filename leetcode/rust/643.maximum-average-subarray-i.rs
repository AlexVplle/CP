// @leet start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k_usize: usize = k as usize;
        let mut sum_sliding_window: i32 = nums[..k_usize].iter().sum();
        let mut start: usize = 0;
        let mut max_sum: i32 = sum_sliding_window;
        for i in k_usize..nums.len() {
            sum_sliding_window += nums[i] - nums[start];
            max_sum = max_sum.max(sum_sliding_window);
            start += 1;
        }
        max_sum as f64 / k as f64
    }
}
// @leet end

