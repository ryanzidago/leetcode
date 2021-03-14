fn main() {
    println!("Hello, world!");
}

fn is_palindrome_00(s: String) -> bool {
    let alphabetic_string = s
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>();

    alphabetic_string == alphabetic_string.chars().rev().collect::<String>()
}

fn is_palindrome_01(mut s: String) -> bool {
    s.make_ascii_lowercase();
    let mut chars = s.chars().filter(char::is_ascii_alphanumeric);

    while let (Some(next), Some(next_back)) = (chars.next(), chars.next_back()) {
        if next != next_back {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_palindrome_test_00() {
        let input = "A man, a plan, a canal: Panama".to_string();
        assert!(is_palindrome_00(input));

        let input = "race a car".to_string();
        assert!(false == is_palindrome_00(input));
    }

    #[test]
    fn is_palindrome_test_01() {
        let input = "A man, a plan, a canal: Panama".to_string();
        assert!(is_palindrome_01(input));

        let input = "race a car".to_string();
        assert!(false == is_palindrome_01(input));
    }
}
