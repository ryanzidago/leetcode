use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut kth_largest: i32 = 0;

    for n in nums.iter() {
        heap.push(*n);
    }

    for _ in 0..k {
        kth_largest = heap.pop().unwrap();
    }

    return kth_largest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_kth_largest_test() {
        assert_eq!(5, find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
        assert_eq!(4, find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4));
        assert_eq!(0, find_kth_largest(vec![0], 1));
        assert_eq!(0, find_kth_largest(vec![0, 0, 0, -1], 2));
    }
}
