impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter().scan(0, |state, x| {
            *state += x;
            Some(*state)
        }).collect()
    }
}
