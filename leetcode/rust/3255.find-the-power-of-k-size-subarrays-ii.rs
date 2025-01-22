// @leet start
impl Solution 
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k_usize: usize = k as usize;
        let len_nums: usize = nums.len();
        let mut streak: usize = 0;
        let mut result: Vec<i32> = Vec::with_capacity(len_nums - k_usize + 1);
        for i in 0..len_nums {
            if i == 0 || nums[i] == nums[i - 1] + 1 {
                streak += 1;
            } else {
                streak = 1;
            }
            if i + 1 >= k_usize {
                if streak >= k_usize {
                    result.push(nums[i]);
                } else {
                    result.push(-1);
                }
            }
        }
        result
    }
}
// @leet end

