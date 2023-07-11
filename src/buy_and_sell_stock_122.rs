mod solution;

pub use solution::Solution;

impl Solution {
    pub fn max_profit(&self, prices: Vec<i32>) -> i32 {
        let mut profits: Vec<i32> = Vec::new();

        let mut i: usize = 0;
        let length: usize = prices.len();

        while i + 1 < length {
            profits.push(prices[i + 1] - prices[i]);
            i += 1;
        }

        let mut r: i32 = 0;
        for profit in profits {
            if profit > 0 {
                r += profit;
            }
        }

        r
    }
}