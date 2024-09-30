impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut write_index: usize = 0;
        for read_index in 0..nums.len() {
            if nums[read_index] != 0 {
                if write_index != read_index {
                    nums.swap(write_index, read_index);
                }
                write_index += 1;
            }
        }
    }
}
