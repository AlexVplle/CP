use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (index, &value) in nums.iter().enumerate() {
            if let Some(&other_index) = hashmap.get(&(target - value)) {
                return vec![index as i32, other_index];
            }
            hashmap.insert(value, index as i32);
        }
        vec![]
    }
}
