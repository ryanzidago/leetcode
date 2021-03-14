fn main() {
    println!("Hello, world!");
}

fn reverse_string(s: &mut Vec<char>) {
    if s.len() > 0 {
        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

#[test]
fn reverse_string_test() {
    let mut input: Vec<char> = vec!['1', '2', '3', '4'];
    let expected: Vec<char> = vec!['4', '3', '2', '1'];

    reverse_string(&mut input);
    assert_eq!(expected, input);
}
