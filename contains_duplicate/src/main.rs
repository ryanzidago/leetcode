use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    nums.sort();
    for i in 0..nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            return true;
        }
    }

    false
}

fn contains_duplicate_bis(nums: Vec<i32>) -> bool {
    let mut found: HashMap<i32, bool> = HashMap::new();

    for i in 0..nums.len() {
        match found.get(&nums[i]) {
            Some(true) => return true,
            _ => found.insert(nums[i], true),
        };
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains_duplicate_test() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn contains_duplicate_bis_test() {
        let nums = vec![1, 2, 3, 1];
        assert!(contains_duplicate_bis(nums));
    }
}
