// @leet start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m: usize = m as usize;
        let mut n: usize = n as usize;
        while m > 0 && n > 0 {
            if nums1[m - 1] > nums2[n - 1] {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
        while n > 0 {
            nums1[n - 1] = nums2[n - 1];
            n -= 1;
        }
    }
}
// @leet end
