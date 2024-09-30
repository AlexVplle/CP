use std::collections::VecDeque;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut vec: VecDeque<i32> = VecDeque::with_capacity(n);
        let mut result: usize = usize::MAX;
        let mut sum: i32 = 0;
        for i in 0..n {
            sum += nums[i];
            vec.push_back(nums[i]);
            if sum >= target {
                while sum - vec.front().unwrap() >= target {
                    sum -= vec.pop_front().unwrap();
                }
                result = result.min(vec.len());
            }
        }
        if result == usize::MAX { 0 } else { result as i32 }
    }
}
