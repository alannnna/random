// EXERCISE 6: &str vs &String — prefer the more flexible type.
// `&String` is a reference to an owned String.
// `&str` is a string slice — it can point into a String, a string literal, or any string data.
// A `&String` can always coerce to `&str`, but `&str` cannot coerce to `&String`.
// Functions that only need to read a string should take `&str` — it works for everything.
// Fix `is_palindrome` and `longest` to accept `&str` instead of `&String`.
// Run: cargo test --bin ex06_str_vs_string -p module_04_borrowing

fn is_palindrome(s: &String) -> bool {   // BUG: too restrictive — should be &str
    let chars: Vec<char> = s.chars().collect();
    let rev: Vec<char> = chars.iter().rev().cloned().collect();
    chars == rev
}

fn longest(a: &String, b: &String) -> usize {   // BUG: should be &str
    if a.len() >= b.len() { a.len() } else { b.len() }
}

fn check_words(words: &[&str]) -> Vec<bool> {
    // This won't compile until is_palindrome accepts &str
    words.iter().map(|w| is_palindrome(w)).collect()
}

fn main() {
    let word = String::from("racecar");
    println!("{} is palindrome: {}", word, is_palindrome(&word));
    // String literals are &str — this won't work with &String parameter:
    println!("longest of 'hi' and 'hello': {}", longest("hi", "hello"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome("level"));
    }

    #[test]
    fn test_check_words() {
        let results = check_words(&["racecar", "hello", "level", "rust"]);
        assert_eq!(results, vec![true, false, true, false]);
    }

    #[test]
    fn test_longest() {
        assert_eq!(longest("hi", "hello"), 5);
        assert_eq!(longest("rust", "go"), 4);
    }
}
