use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn single_number_00(nums: Vec<i32>) -> i32 {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    for n in nums.iter() {
        match hashmap.get(n) {
            Some(m) => hashmap.insert(*n, m + 1),
            None => hashmap.insert(*n, 1),
        };
    }

    for (key, value) in hashmap {
        if value == 1 {
            return key;
        }
    }

    unreachable!()
}

fn single_number_00_bis(nums: Vec<i32>) -> i32 {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    for n in nums.iter() {
        hashmap.entry(*n).and_modify(|m| *m += 1).or_insert(1);
    }

    for (key, value) in hashmap {
        if value == 1 {
            return key;
        }
    }

    unreachable!()
}

fn single_number_01(nums: Vec<i32>) -> i32 {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    for n in nums.iter() {
        match hashmap.remove(&n) {
            Some(_) => None,
            None => hashmap.insert(*n, 1),
        };
    }

    *hashmap.keys().next().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_number_00_test() {
        let input = vec![2, 2, 1];
        assert_eq!(single_number_00(input), 1);

        let input = vec![4, 1, 2, 1, 2];
        assert_eq!(single_number_00(input), 4);

        let input = vec![1];
        assert_eq!(single_number_00(input), 1);
    }

    #[test]
    fn test_name() {
        let input = vec![2, 2, 1];
        assert_eq!(single_number_00_bis(input), 1);

        let input = vec![4, 1, 2, 1, 2];
        assert_eq!(single_number_00_bis(input), 4);

        let input = vec![1];
        assert_eq!(single_number_00_bis(input), 1);
    }

    #[test]
    fn single_number_01_test() {
        let input = vec![2, 2, 1];
        assert_eq!(single_number_01(input), 1);

        let input = vec![4, 1, 2, 1, 2];
        assert_eq!(single_number_01(input), 4);

        let input = vec![1];
        assert_eq!(single_number_01(input), 1);
    }
}
