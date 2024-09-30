impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let total_sum: i32 = nums.iter().sum();
        let mut left_sum: i32 = 0;
        let mut result: Vec<i32> = Vec::with_capacity(nums.len());
        for &num in &nums {
            let right_sum: i32 = total_sum - left_sum - num;
            result.push((left_sum - right_sum).abs());
            left_sum += num;
        }
        result
    }
}
