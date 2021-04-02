use std::cmp;

fn main() {
    println!("Hello, world!");
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut current_sum: i32 = 0;
    let mut best_sum: i32 = 0;

    for i in 1..prices.len() {
        current_sum += prices[i] - prices[i - 1];
        current_sum = cmp::max(0, current_sum);
        best_sum = cmp::max(best_sum, current_sum);
    }

    return best_sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn max_profit_test() {
        assert_eq!(5, max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(1, max_profit(vec![1, 2]));
        assert_eq!(0, max_profit(vec![5, 4, 3, 2, 1]));
    }
}
