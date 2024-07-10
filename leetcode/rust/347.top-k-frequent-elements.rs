use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hashmap : HashMap<i32, u64> = HashMap::new();
        nums.into_iter().for_each(|num| {
            hashmap.entry(num).and_modify(|value| *value += 1).or_insert(1);
        });
        let mut heap : BinaryHeap<Reverse<(u64, i32)>> = BinaryHeap::with_capacity(k as usize);
        hashmap.into_iter().for_each(|(key, value)| {
            heap.push(Reverse((value, key)));
            if heap.len() > k as usize {
                heap.pop();
            }
        });
        heap.into_iter().map(|Reverse((_, value))| value).collect()
    }
}
