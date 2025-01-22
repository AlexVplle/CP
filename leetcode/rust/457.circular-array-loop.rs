// @leet start
use std::collections::HashSet;

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let nums_len: usize = nums.len();
        let mut visited: HashSet<usize> = HashSet::with_capacity(nums_len);
        for start_index in 0..nums_len {
            if visited.contains(&start_index) {
                continue;
            }
            let first_step: i32 = nums[start_index];
            visited.insert(start_index);
            let mut slow_index: usize =
                ((start_index as i32 + (nums[start_index] % nums_len as i32) + nums_len as i32)
                    % nums_len as i32) as usize;
            if first_step * nums[slow_index] >= 0 {
                visited.insert(slow_index);
                let mut fast_index: usize =
                    ((slow_index as i32 + (nums[slow_index] % nums_len as i32) + nums_len as i32)
                        % nums_len as i32) as usize;
                if slow_index != fast_index {
                    visited.insert(fast_index);
                    let mut valid: bool = true;
                    while slow_index != fast_index {
                        if first_step * nums[slow_index] < 0
                            || slow_index
                                == ((slow_index as i32
                                    + (nums[slow_index] % nums_len as i32)
                                    + nums_len as i32)
                                    % nums_len as i32) as usize
                        {
                            valid = false;
                            break;
                        }
                        slow_index = ((slow_index as i32
                            + (nums[slow_index] % nums_len as i32)
                            + nums_len as i32)
                            % nums_len as i32) as usize;
                        visited.insert(slow_index);
                        if first_step * nums[fast_index] < 0
                            || fast_index
                                == ((fast_index as i32
                                    + (nums[fast_index] % nums_len as i32)
                                    + nums_len as i32)
                                    % nums_len as i32) as usize
                        {
                            valid = false;
                            break;
                        }
                        fast_index = ((fast_index as i32
                            + (nums[fast_index] % nums_len as i32)
                            + nums_len as i32)
                            % nums_len as i32) as usize;
                        visited.insert(fast_index);
                        if first_step * nums[fast_index] < 0
                            || fast_index
                                == ((fast_index as i32
                                    + (nums[fast_index] % nums_len as i32)
                                    + nums_len as i32)
                                    % nums_len as i32) as usize
                        {
                            valid = false;
                            break;
                        }
                        fast_index = ((fast_index as i32
                            + (nums[fast_index] % nums_len as i32)
                            + nums_len as i32)
                            % nums_len as i32) as usize;
                        visited.insert(fast_index);
                    }
                    if valid {
                        return true;
                    }
                }
            }
        }
        false
    }
}
// @leet end
