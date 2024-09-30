impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let total_sum: i32 = n * (n + 1) / 2;
        let x = (total_sum as f64).sqrt() as i32;
        if x * x == total_sum {
            return x;
        } else {
            return -1;
        }
    }
}
