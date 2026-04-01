// EXERCISE 6: Moving out of a loop.
// `for item in collection` moves each item out of the collection.
// After the loop, the collection is gone. Fix this in two places:
// 1. Save `words.len()` before the loop.
// 2. Use `words.iter()` instead of `for word in words` to borrow instead of move.
// Run: cargo test --bin ex06_move_in_loop -p module_03_ownership

fn total_length(words: Vec<String>) -> (usize, usize) {
    let mut total_chars = 0;

    for word in words {         // this moves each String out of the Vec
        total_chars += word.len();
    }

    let word_count = words.len();
    (word_count, total_chars)
}

fn main() {
    let words = vec![
        String::from("hello"),
        String::from("world"),
        String::from("foo"),
    ];
    let (count, chars) = total_length(words);
    println!("{} words, {} total characters", count, chars);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_length() {
        let words = vec![
            String::from("hello"),
            String::from("world"),
        ];
        let (count, chars) = total_length(words);
        assert_eq!(count, 2);
        assert_eq!(chars, 10);
    }
}
