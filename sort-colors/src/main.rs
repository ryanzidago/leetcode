fn main() {
    println!("Hello, world!");
}

fn sort_colors(nums: &mut Vec<i32>) {
    let length = nums.len();

    if length <= 1 {
        return;
    }

    let mut unsorted_until_index = length - 1;
    let mut sorted = false;

    while !sorted {
        sorted = true;
        for i in 0..unsorted_until_index {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                sorted = false;
            }
        }

        unsorted_until_index -= 1;
    }
}

fn sort_colors_bis(nums: &mut Vec<i32>) {
    let length = nums.len();

    if length <= 1 {
        return;
    }
    let mut red: usize = 0;
    let mut white: usize = 0;
    let mut blue: usize = length - 1;

    while white <= blue {
        if nums[white] == 0 {
            nums.swap(red, white);
            red += 1;
            white += 1;
        } else if nums[white] == 1 {
            white += 1;
        } else {
            nums.swap(white, blue);
            blue -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort_colors_test() {
        let mut nums: Vec<i32> = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], nums);

        let mut nums: Vec<i32> = vec![2, 0, 1];
        sort_colors(&mut nums);
        assert_eq!(vec![0, 1, 2], nums);

        let mut nums: Vec<i32> = vec![0];
        sort_colors(&mut nums);
        assert_eq!(vec![0], nums);
    }

    #[test]
    fn sort_colors_bis_test() {
        let mut nums: Vec<i32> = vec![2, 0, 2, 1, 1, 0];
        sort_colors_bis(&mut nums);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], nums);

        let mut nums: Vec<i32> = vec![2, 0, 1];
        sort_colors_bis(&mut nums);
        assert_eq!(vec![0, 1, 2], nums);

        let mut nums: Vec<i32> = vec![0];
        sort_colors_bis(&mut nums);
        assert_eq!(vec![0], nums);

        let mut nums: Vec<i32> = vec![2];
        sort_colors_bis(&mut nums);
        assert_eq!(vec![2], nums);
    }
}
