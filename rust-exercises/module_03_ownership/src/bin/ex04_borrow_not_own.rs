// EXERCISE 4: Borrow instead of taking ownership.
// These functions only need to READ the string — they don't need to own it.
// Change both function signatures to borrow (&str) instead of taking ownership (String).
// This lets the caller keep using the value after the call.
// Run: cargo test --bin ex04_borrow_not_own -p module_03_ownership

fn word_count(text: String) -> usize {   // BUG: takes ownership, should borrow
    text.split_whitespace().count()
}

fn char_count(text: String) -> usize {   // BUG: takes ownership, should borrow
    text.chars().count()
}

fn analyze(text: String) -> (usize, usize) {
    let words = word_count(text);   // `text` is moved here
    let chars = char_count(text);   // BUG: `text` already moved
    (words, chars)
}

fn main() {
    let passage = String::from("the quick brown fox");
    let (words, chars) = analyze(passage);
    println!("Words: {}, Chars: {}", words, chars);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze() {
        let text = String::from("hello world foo");
        let (words, chars) = analyze(text);
        assert_eq!(words, 3);
        assert_eq!(chars, 15);
    }
}
