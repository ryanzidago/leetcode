fn main() {
    println!("Hello, world!");
}

fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    nums.iter()
        .map(|num| {
            sum += num;
            sum
        })
        .collect()
}

#[test]
fn running_sum_test() {
    let nums: Vec<i32> = vec![1, 2, 3, 4];
    let expected: Vec<i32> = vec![1, 3, 6, 10];

    let outcome: Vec<i32> = running_sum(nums);
    assert_eq!(outcome, expected)
}
