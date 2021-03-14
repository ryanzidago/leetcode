const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    println!("Hello, world!");
}

fn halves_are_alike(s: String) -> bool {
    let lowercased = s.to_lowercase();
    let (first, last) = lowercased.split_at(lowercased.len() / 2);

    let mut first = first.to_string();
    let mut last = last.to_string();

    first.retain(|char| is_vowel(&char));
    last.retain(|char| is_vowel(&char));

    first.len() == last.len()
}

fn is_vowel(char: &char) -> bool {
    VOWELS.contains(char)
}

#[test]
fn halves_are_alike_test() {
    assert!(halves_are_alike(String::from("book")));
    assert!(halves_are_alike("AbCdEfGh".to_string()));
    assert!(halves_are_alike("Uo".to_string()));

    assert!(false == halves_are_alike(String::from("textbook")));
    assert!(false == halves_are_alike(String::from("MerryChristmas")));
}

#[test]
fn is_vowel_test() {
    for vowel in VOWELS.iter() {
        assert!(is_vowel(vowel));
    }

    for consonant in ['q', 't', 'x', 'y', 'p'].iter() {
        assert!(false == is_vowel(consonant));
    }
}
