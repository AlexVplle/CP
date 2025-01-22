// @leet start
impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k_usize = k as usize;
        let len_results = nums.len() - k_usize + 1;
        let mut results = Vec::with_capacity(len_results);

        for i in 0..len_results {
            let subarray = &nums[i..i + k_usize];
            let mut contiguous = true;

            for j in 1..k_usize {
                if subarray[j] != subarray[j - 1] + 1 {
                    contiguous = false;
                    break;
                }
            }

            if contiguous {
                results.push(subarray[k_usize - 1]);
            } else {
                results.push(-1);
            }
        }

        results
    }
}

// @leet end

