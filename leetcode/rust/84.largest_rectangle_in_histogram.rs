impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        heights.push(0);
        let n: usize = heights.len();
        let mut max_area: i32 = 0;
        let mut stack: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            while let Some(&top) = stack.last() {
                if heights[i] < heights[top] {
                    stack.pop();
                    let width: usize = if stack.is_empty() {
                        i
                    } else {
                        i - stack.last().unwrap() - 1
                    };
                    max_area = max_area.max(width as i32 * heights[top]);
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        max_area
    }
}
