fn main() {
    println!("Hello, world!");
}

fn length_of_last_word_00(word: String) -> i32 {
    let word = word.trim();
    if word.is_empty() {
        0 as i32
    } else {
        word.split(" ").last().unwrap().chars().count() as i32
    }
}

#[test]
fn length_of_last_word_00_test() {
    let word: String = String::from("Hello World");
    assert_eq!(5, length_of_last_word_00(word));

    let word: String = String::from("Hello ");
    assert_eq!(5, length_of_last_word_00(word));
}
