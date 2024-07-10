impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result : Vec<i32> = Vec::with_capacity(nums.len());
        let mut sum : i32 = 1;
        for num in &nums {
            result.push(sum);
            sum *= num;
        }
        sum = 1;
        for i in (0..nums.len()).rev() {
            result[i] *= sum;
            sum *= nums[i];
        }
        result
    }
}
