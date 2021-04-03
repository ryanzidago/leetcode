use std::cmp;

fn main() {}

// Kadane's algorithm
// time complexity: O(n)
// space complexity: O(1)
fn kadane(nums: Vec<i32>) -> i32 {
    let mut current_subarray: i32 = nums[0];
    let mut max_subarray: i32 = nums[0];

    for num in &nums[1..] {
        current_subarray = cmp::max(*num, current_subarray + *num);
        max_subarray = cmp::max(max_subarray, current_subarray);
    }

    return max_subarray;
}

// brute force approach
// O(n²) time complexity
// exceeds Leetcode's time limit
fn brute_force(nums: Vec<i32>) -> i32 {
    let length = nums.len();

    if length == 1 {
        return nums[0];
    }

    let mut sum_of_subarrays: Vec<i32> = vec![];

    for i in 0..(length) {
        for j in i..(length) {
            let sum_of_subarray: &i32 = &nums[i..=j].iter().sum();
            sum_of_subarrays.push(*sum_of_subarray);
        }
    }

    *sum_of_subarrays.iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn brute_force_test() {
        assert_eq!(6, brute_force(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
        assert_eq!(1, brute_force(vec![1]));
        assert_eq!(23, brute_force(vec![5, 4, -1, 7, 8]));
        assert_eq!(1, brute_force(vec![-2, 1]));
    }

    #[test]
    fn kadane_test() {
        assert_eq!(6, kadane(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
        assert_eq!(1, kadane(vec![1]));
        assert_eq!(23, kadane(vec![5, 4, -1, 7, 8]));
        assert_eq!(1, kadane(vec![-2, 1]));
        assert_eq!(3, kadane(vec![1, 2, -5, 2]));
    }
}
