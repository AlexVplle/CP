// @leet start

use std::collections::HashMap;

impl Solution {
    pub fn find_maximum_xor(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut hashmap: HashMap<i32, i32> = HashMap::with_capacity(32);
        for i in nums {
            println!("{}", 1 << (31 - i.leading_zeros()));
        }
        0
    }
}
// @leet end

