impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n: usize = n as usize;
        let mut result: Vec<String> = Vec::new();
        let mut sequence: Vec<u8> = vec![b'('; 2 * n];
        for i in 0..(1 << (2 * n)) {
            if Self::is_valid(i, n) {
                for j in 0..(2 * n) {
                    sequence[j] = if (i & (1 << j)) == 0 { b'(' } else { b')' };
                }
                result.push(String::from_utf8(sequence.clone()).unwrap()) 
            }
        }
        result
    }

    fn is_valid(mut x: i32, n: usize) -> bool {
        let mut sum: i8 = 0;
        let mut count: usize = 0;
        while x > 0 {
            sum += if x & 1 == 1 { -1 } else { 1 };
            if sum < 0 {
                return false;
            }
            count += 1;
            x >>= 1;
        }
        sum == 0 && count == 2 * n
    }
}

