// @leet start
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut current_sum = 0;
        let mut hashmap = HashMap::new();
        hashmap.insert(0, 1);

        for num in nums {
            current_sum += num;

            if let Some(&occurrence) = hashmap.get(&(current_sum - k)) {
                count += occurrence;
            }

            *hashmap.entry(current_sum).or_insert(0) += 1;
        }

        count
    }
}
// @leet end

