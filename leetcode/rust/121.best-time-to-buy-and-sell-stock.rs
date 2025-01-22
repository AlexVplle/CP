// @leet start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price_option: Option<i32> = None;
        let mut result: i32 = 0;
        for price in prices {
            if let Some(min_price) = min_price_option {
                if min_price > price {
                    min_price_option = Some(price)
                }
                result = result.max(price - min_price);
            } else {
                min_price_option = Some(price)
            }
        }
        result
    }
}
// @leet end

