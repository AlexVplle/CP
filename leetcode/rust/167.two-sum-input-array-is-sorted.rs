impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut start, mut end): (usize, usize) = (0, numbers.len() - 1);
        while start < end {
            let sum: i32 = numbers[start] + numbers[end];
            if sum == target {
                return vec![start as i32 + 1, end as i32 + 1];
            } else if sum < target {
                start += 1;
            } else {
                end -= 1;
            }
        }
        vec![]
    }
}
