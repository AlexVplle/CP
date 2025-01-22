// @leet start
impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut number_of_bit: u32 = num2.count_ones();
        let mut result: i32 = 0;
        for bit in (0..32).rev() {
            let mask: i32 = 1 << bit;
            if num1 & mask != 0 {
                result |= mask;
                number_of_bit -= 1;
            }
            if number_of_bit == 0 {
                return result;
            }
        }
        for bit in 0..32 {
            if number_of_bit == 0 {
                break;
            }
            let mask: i32 = 1 << bit;
            if result & mask == 0 {
                result |= mask;
                number_of_bit -= 1;
            }
        }
        result
    }
}
// @leet end
