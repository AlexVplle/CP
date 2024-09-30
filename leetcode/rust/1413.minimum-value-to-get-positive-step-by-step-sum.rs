impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        (- nums.into_iter().fold((i32::MAX, 0), |(min_value, current_value), change| {
            let new_value: i32 = current_value + change;
            (min_value.min(new_value), new_value)
        }).0 + 1).max(1) 
    }
}
