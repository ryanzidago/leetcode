use std::cmp;

fn main() {
    println!("Hello, world!");
}

fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let mut profit: i32 = 0;
    let mut hold = -prices[0];

    for i in 1..prices.len() {
        profit = cmp::max(profit, hold + prices[i] - fee);
        hold = cmp::max(hold, profit - prices[i]);
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_profit_test() {
        let prices = vec![1, 3, 7, 5, 10, 3];
        let profit = max_profit(prices, 3);
        assert_eq!(profit, 6);

        let prices = vec![1, 3, 2, 8, 4, 9];
        let profit = max_profit(prices, 2);
        assert_eq!(profit, 8);
    }
}
