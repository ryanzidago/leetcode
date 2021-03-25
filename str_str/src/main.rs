fn main() {
    println!("Hello, world!");
}

fn str_str(haystack: String, needle: String) -> i32 {
    let v: Vec<_> = haystack.match_indices(&needle).collect();
    match v.first() {
        Some((index, _needle)) => *index as i32,
        None => -1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn str_str_test() {
        let haystack: String = "hello".to_string();
        let needle: String = "ll".to_string();

        assert_eq!(2, str_str(haystack, needle));
    }
}
