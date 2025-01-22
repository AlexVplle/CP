// @leet start
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let n: usize = arr.len();
        let mid: i32 = (n as i32 + 1) / 2;
        let mut vec: Vec<i32> = vec![mid; n];
        for i in 1..mid {
            let i: usize = i as usize;
            vec[i] = vec[i - 1] + 1;
            vec[n - i - 1] = vec[n - i] + 1;
        }
        for i in &vec {
            print!("{i} ");
        }
        vec.into_iter().zip(arr.into_iter()).map(|(x, y)| x * y).sum()
    }
}
// @leet end

