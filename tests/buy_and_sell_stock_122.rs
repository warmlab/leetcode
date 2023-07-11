#[cfg(test)]
#[path="../src/buy_and_sell_stock_122.rs"]

mod buy_and_sell_stock_122;

mod test_buy_and_sell_stock {
    use super::buy_and_sell_stock_122::Solution;

    #[test]
    fn test_buy_and_sell_stock_1() {
        let s: Solution = Solution;

        let prices: Vec<i32> = vec![7,1,5,3,6,4];

        let profit = s.max_profit(prices);

        assert_eq!(profit, 7);
    }

    #[test]
    fn test_buy_and_sell_stock_2() {
        let s: Solution = Solution;

        let prices: Vec<i32> = vec![1,2,3,4,5];

        let profit = s.max_profit(prices);

        assert_eq!(profit, 4);
    }

    #[test]
    fn test_buy_and_sell_stock_3() {
        let s: Solution = Solution;

        let prices: Vec<i32> = vec![7,6,4,3,1];

        let profit = s.max_profit(prices);

        assert_eq!(profit, 0);
    }
}