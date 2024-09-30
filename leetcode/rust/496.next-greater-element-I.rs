use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len_nums2: usize = nums2.len();
        let mut stack: Vec<i32> = Vec::with_capacity(len_nums2);
        let mut hashmap: HashMap<i32, i32> = HashMap::with_capacity(len_nums2);
        nums2.iter().for_each(|value: &i32| {
            while let Some(&top) = stack.last() {
                if top < *value {
                    hashmap.insert(top, *value);
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(*value);
        });
        nums1
            .into_iter()
            .map(|value: i32| *hashmap.get(&value).unwrap_or(&-1))
            .collect()
    }
}
