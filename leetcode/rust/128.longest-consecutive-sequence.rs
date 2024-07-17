use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums_set: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        nums_set
            .iter()
            .filter(|num: &&i32| !nums_set.contains(&(*num - 1)))
            .map(|start: &i32| {
                (1..)
                    .take_while(|length: &i32| nums_set.contains(&(start + length)))
                    .last()
                    .map_or(1, |last: i32| last + 1)
            })
            .max()
            .unwrap_or(0)
    }
}
