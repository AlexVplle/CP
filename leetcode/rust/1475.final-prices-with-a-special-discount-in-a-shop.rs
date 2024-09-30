impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let n: usize = prices.len();
        let mut result: Vec<i32> = prices.clone();
        let mut stack: Vec<usize> = Vec::with_capacity(n);
        (0..n).into_iter().for_each(|index: usize| {
            while let Some(&top) = stack.last() {
                if prices[top] >= prices[index] {
                    result[top] -= prices[index];
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(index);
        });
        result
    }
}
