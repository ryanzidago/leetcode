fn main() {
    println!("Hello, world!");
}

fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for char in s.chars() {
        match char {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '}' | ')' | ']' if Some(char) != stack.pop() => return false,
            _ => (),
        }
    }
    return stack.is_empty();
}

#[test]
fn is_valid_test() {
    let input: String = String::from("()");
    assert!(is_valid(input));

    let input: String = String::from("()[]{}");
    assert!(is_valid(input));

    let input: String = String::from("(]");
    assert!(!is_valid(input));
}
