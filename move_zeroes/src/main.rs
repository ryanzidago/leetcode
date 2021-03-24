fn main() {
    println!("Hello, world!");
}

fn move_zeroes(numbers: &mut Vec<i32>) {
    let number_of_zeroes = numbers.iter().filter(|&n| *n == 0).count();
    numbers.retain(|&n| n != 0);

    for _ in 0..number_of_zeroes {
        numbers.push(0);
    }
}

fn move_zeroes_bis(numbers: &mut Vec<i32>) {
    numbers.sort_by_key(|&n| n == 0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn move_zeroes_test() {
        let mut input = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut input);
        assert_eq!(vec![1, 3, 12, 0, 0], input);
    }

    #[test]
    fn move_zeroes_bis_test() {
        let mut input = vec![0, 1, 0, 3, 12];
        move_zeroes_bis(&mut input);
        assert_eq!(vec![1, 3, 12, 0, 0], input);
    }
}
