use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    println!("Hello, world!");
}

#[derive(Eq, Ord, PartialEq)]
struct FrequentWord {
    word: String,
    frequency: usize,
}

impl FrequentWord {
    fn new(word: String, frequency: usize) -> FrequentWord {
        FrequentWord { word, frequency }
    }
}

impl PartialOrd for FrequentWord {
    fn partial_cmp(&self, other: &FrequentWord) -> Option<Ordering> {
        Some(match self.frequency.cmp(&other.frequency) {
            Ordering::Equal => other.word.cmp(&self.word),
            ordering => {
                println!("ordering = {:#?}", ordering);
                ordering
            }
        })
    }
}

fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut heap: BinaryHeap<FrequentWord> = BinaryHeap::with_capacity(k as usize);
    let mut result: Vec<String> = vec![];

    for word in words {
        if let Some(frequency) = map.get_mut(&word) {
            *frequency += 1;
        } else {
            map.insert(word, 1);
        }
    }

    for (word, frequency) in map {
        let frequent_word = FrequentWord::new(word, frequency);
        heap.push(frequent_word);
    }

    for _ in 0..k {
        let word: String = heap.pop().unwrap().word;
        result.push(word);
    }

    println!("{:#?}", result);

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn top_k_frequent_test() {
        let words = vec![
            "the".to_string(),
            "day".to_string(),
            "is".to_string(),
            "sunny".to_string(),
            "the".to_string(),
            "the".to_string(),
            "the".to_string(),
            "sunny".to_string(),
            "is".to_string(),
            "is".to_string(),
        ];
        let k = 4;

        let expected = vec![
            "the".to_string(),
            "is".to_string(),
            "sunny".to_string(),
            "day".to_string(),
        ];

        let output = top_k_frequent(words, k);

        assert_eq!(expected, output);
    }
}
