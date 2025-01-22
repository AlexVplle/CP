// @leet start
const MOD: i32 = 1_000_000_007;

impl Solution {
    fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let high: usize = high as usize;
        let zero: usize = zero as usize;
        let one: usize = one as usize;
        let mut dynamic_programming: Vec<i32> = vec![0; high + 1];
        dynamic_programming[0] = 1;
        for i in 0..=high {
            if i >= zero {
                dynamic_programming[i] =
                    (dynamic_programming[i] + dynamic_programming[i - zero]) % MOD;
            }
            if i >= one {
                dynamic_programming[i] =
                    (dynamic_programming[i] + dynamic_programming[i - one]) % MOD;
            }
        }
        let mut result: i32 = 0;
        for i in low as usize..=high {
            result = (result + dynamic_programming[i]) % MOD;
        }
        result
    }
}
// @leet end

