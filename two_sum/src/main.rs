use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn two_sum_00(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    unreachable!()
}

fn two_sum_01(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut compliments: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        match compliments.get(&nums[i]) {
            Some(&j) => return vec![j, i as i32],
            None => compliments.insert(target - nums[i], i as i32),
        };
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_00_test() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        let expected: Vec<i32> = vec![0, 1];
        let outcome: Vec<i32> = two_sum_00(nums, target);

        assert_eq!(expected, outcome)
    }

    #[test]
    fn two_sum_01_test() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        let expected: Vec<i32> = vec![0, 1];
        let outcome: Vec<i32> = two_sum_01(nums, target);

        assert_eq!(expected, outcome)
    }
}
