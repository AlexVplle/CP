// @leet start
use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut fruits_count: HashMap<i32, usize> = HashMap::with_capacity(3);
        let mut start: usize = 0;
        let mut max_length: i32 = 0;
        for end in 0..fruits.len() {
            *fruits_count.entry(fruits[end]).or_default() += 1;
            while fruits_count.len() > 2 {
                let fruit_at_start: i32 = fruits[start];
                *fruits_count.get_mut(&fruit_at_start).unwrap() -= 1;
                if fruits_count[&fruit_at_start] == 0 {
                    fruits_count.remove(&fruit_at_start);
                }
                start += 1;
            }
            max_length = max_length.max((end - start) as i32 + 1);
        }
        max_length
    }
}
// @leet end

