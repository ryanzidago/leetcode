use std::cmp;

fn main() {
    println!("Hello, world!");
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit: i32 = 0;
    let mut min_price: i32 = prices[0];

    for price in prices {
        if price < min_price {
            min_price = price;
        } else {
            profit = cmp::max(price - min_price, profit)
        }
    }

    return profit;
}

fn kadane(prices: Vec<i32>) -> i32 {
    let mut current_sum: i32 = 0;
    let mut best_sum: i32 = 0;

    for i in 1..prices.len() {
        current_sum += prices[i] - prices[i - 1];
        current_sum = cmp::max(0, current_sum);
        best_sum = cmp::max(best_sum, current_sum);
    }

    return best_sum;
}

fn brute_force(prices: Vec<i32>) -> i32 {
    let mut max_profit: i32 = 0;

    for i in 0..(prices.len() - 1) {
        for j in (i + 1)..prices.len() {
            let profit: i32 = prices[j] - prices[i];

            if profit > max_profit {
                max_profit = profit;
            }
        }
    }

    return max_profit;
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

    #[test]
    fn kadane_test() {
        assert_eq!(5, kadane(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(1, kadane(vec![1, 2]));
        assert_eq!(0, kadane(vec![5, 4, 3, 2, 1]));
        assert_eq!(3, kadane(vec![1, 2, 3, 4]));
        assert_eq!(1, kadane(vec![1, 2, 1, 2]));
        assert_eq!(3, kadane(vec![1, 2, 3, 4, 3, 2]));
    }

    #[test]
    fn brute_force_test() {
        assert_eq!(5, brute_force(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(1, brute_force(vec![1, 2]));
        assert_eq!(0, brute_force(vec![5, 4, 3, 2, 1]));
    }
}
