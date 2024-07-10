use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let hash_set: HashSet<i32> = nums.iter().cloned().collect();
        nums.len() != hash_set.len()
    }
}
