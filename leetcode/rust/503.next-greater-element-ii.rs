// @leet start
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let len: usize = nums.len();
        let mut result: Vec<i32> = vec![-1; len];
        let mut stack: Vec<i32> = Vec::with_capacity(2 * len);
        (0..2 * len).rev().for_each(|i: usize| {
            let modulo_i: usize = i % len;
            let num: i32 = nums[modulo_i];
            while let Some(&top) = stack.last() {
                if top <= num {
                    stack.pop();
                } else {
                    result[modulo_i] = top;
                    break;
                }
            }
            stack.push(num)
        });
        result
    }
}
// @leet end
