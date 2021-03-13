fn main() {
    println!("Hello, world!");
}

fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts
        .iter()
        .map(|account| account.iter().sum())
        .max()
        .unwrap()
}

#[test]
fn maximum_wealth_test() {
    let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
    assert_eq!(6, maximum_wealth(accounts));
}
