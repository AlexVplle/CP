// @leet start
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k: usize = k as usize;
        let n: usize = nums.len();
        let mut deque: VecDeque<usize> = VecDeque::with_capacity(k);
        let mut result: Vec<i32> = Vec::with_capacity(n - k + 1);
        for i in 0..n {
            while !deque.is_empty() && deque.front().unwrap() + k <= i {
                deque.pop_front();
            }
            while !deque.is_empty() && nums[*deque.back().unwrap()] <= nums[i] {
                deque.pop_back();
            }
            deque.push_back(i);
            if i >= k - 1 {
                result.push(nums[*deque.front().unwrap()]);
            }
        }
        result
    }
}
// @leet end

