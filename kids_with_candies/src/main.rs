fn main() {
    println!("Hello, world!");
}

fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = candies.iter().max().unwrap();
    candies
        .iter()
        .map(|candy| &(candy + extra_candies) >= &max)
        .collect::<Vec<bool>>()
}

#[test]
fn kids_with_candies_test() {
    let candies: Vec<i32> = vec![2, 3, 5, 1, 3];
    let extra_candies: i32 = 3;
    assert_eq!(
        vec![true, true, true, false, true],
        kids_with_candies(candies, extra_candies)
    );
}
