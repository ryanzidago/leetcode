use std::cmp;

fn main() {
    println!("Hello, world!");
}

fn max_profit_00(prices: Vec<i32>) -> i32 {
    let mut profit = 0;

    for i in 1..prices.len() {
        profit += cmp::max(prices[i] - prices[i - 1], 0)
    }

    return profit;
}

fn max_profit_01(prices: Vec<i32>) -> i32 {
    let mut total = 0;
    let len = prices.len();

    for i in 0..(len - 1) {
        if prices[i] < prices[i + 1] {
            total += prices[i + 1] - prices[i]
        }
    }

    return total;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn max_profit_00_test() {
        assert_eq!(7, max_profit_00(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(4, max_profit_00(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, max_profit_00(vec![7, 6, 4, 3, 1]));
        assert_eq!(3, max_profit_00(vec![1, 2, 3, 4]));
        assert_eq!(2, max_profit_00(vec![1, 2, 1, 2]));
    }

    #[test]
    fn max_profit_01_test() {
        assert_eq!(7, max_profit_01(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(4, max_profit_01(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, max_profit_01(vec![7, 6, 4, 3, 1]));
        assert_eq!(3, max_profit_01(vec![1, 2, 3, 4]));
        assert_eq!(2, max_profit_01(vec![1, 2, 1, 2]));
    }
}
