fn main() {
    println!("Hello, world!");
}

fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let (left, right) = nums.split_at(n as usize);

    left.iter()
        .zip(right.iter())
        .fold(vec![], |mut acc, (l, r)| {
            acc.push(*l);
            acc.push(*r);
            acc
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shuffle_test() {
        assert_eq!(vec![2, 3, 5, 4, 1, 7], shuffle(vec![2, 5, 1, 3, 4, 7], 3));
    }
}
