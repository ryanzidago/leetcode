use std::collections::{BinaryHeap, HashMap};

fn main() {
    println!("Hello, world!");
}

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::with_capacity(k as usize);
    let mut result: Vec<i32> = Vec::new();

    for num in nums {
        if let Some(frequency) = map.get_mut(&num) {
            *frequency += 1;
        } else {
            map.insert(num, 1);
        }
    }

    for (num, frequency) in map {
        heap.push((frequency, num));
    }

    for _ in 0..k {
        let (_frequency, num): (i32, i32) = heap.pop().unwrap();
        result.push(num);
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn top_k_frequent_test() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let expected = vec![1, 2];
        assert_eq!(expected, top_k_frequent(nums, k));

        let nums = vec![1];
        let k = 1;
        let expected = vec![1];
        assert_eq!(expected, top_k_frequent(nums, k));

        let nums = vec![-1, -1];
        let k = 1;
        let expected = vec![-1];
        assert_eq!(expected, top_k_frequent(nums, k));
    }
}
