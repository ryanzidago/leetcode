fn main() {
    println!("Hello, world!");
}

fn is_anagram_00(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s = s.chars().collect::<Vec<char>>();
    let mut t = t.chars().collect::<Vec<char>>();

    s.sort();
    t.sort();

    s == t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_anagram_00_test() {
        let input_00 = "anagram".to_string();
        let input_01 = "nagaram".to_string();

        assert!(is_anagram_00(input_00, input_01));

        let input_00 = "rat".to_string();
        let input_01 = "car".to_string();

        assert!(false == is_anagram_00(input_00, input_01));

        let input_00 = "éhéh".to_string();
        let input_01 = "hhéé".to_string();

        assert!(is_anagram_00(input_00, input_01));

        let input_00 = "wälder".to_string();
        let input_01 = "derwäl".to_string();

        assert!(is_anagram_00(input_00, input_01));
    }
}
