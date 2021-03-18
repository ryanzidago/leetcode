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
}
