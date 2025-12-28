
//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut l = 0;

        for r in 1..prices.len() {
            if prices[r] < prices[l] {
                l = r;
                continue;
            }

            let current_profit = prices[r] - prices[l];
            max_profit = max_profit.max(current_profit);
        }

        max_profit
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)