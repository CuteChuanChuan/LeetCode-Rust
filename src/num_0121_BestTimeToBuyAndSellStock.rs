
//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = prices[0];

        for &price in &prices[1..] {
            min_price = min_price.min(price);
            max_profit = max_profit.max(price - min_price);
        }

        max_profit
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)