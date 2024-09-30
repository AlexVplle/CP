impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right): (usize, usize) = (0, height.len() - 1);
        let (mut left_max, mut right_max): (i32, i32) = (height[left], height[right]);
        let mut max_area: i32 = 0;
        while left < right {
            let width: i32 = (right - left) as i32;
            if left_max < right_max {
                max_area = max_area.max(width * height[left]);
                left += 1;
                left_max = height[left];
            } else {
                max_area = max_area.max(width * height[right]);
                right -= 1;
                right_max = height[right];
            }
        }
        max_area
    }
}
