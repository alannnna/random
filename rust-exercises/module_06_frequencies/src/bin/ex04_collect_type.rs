// EXERCISE 4: Fix the ambiguous .collect() type.
// `.collect()` can produce many different collection types (Vec, HashSet, HashMap, String...).
// When the type isn't clear from context, you must annotate it explicitly.
// Fix the two functions by adding type annotations so the compiler knows what to collect into.
// Run: cargo test --bin ex04_collect_type -p module_06_frequencies

use std::collections::HashSet;

fn unique_chars(s: &str) -> HashSet<char> {
    s.chars().collect()  // fine — return type tells the compiler what to collect into
}

fn words_starting_with(text: &str, letter: char) -> _ {   // BUG: `_` is not a valid return type
    text.split_whitespace()
        .filter(|w| w.starts_with(letter))
        .collect()
}

fn pair_with_length(words: &[&str]) -> _ {   // BUG: `_` is not a valid return type
    // Should collect into Vec<(&&str, usize)> or Vec<(&str, usize)>
    words
        .iter()
        .map(|w| (*w, w.len()))
        .collect()
}

fn main() {
    println!("{:?}", unique_chars("hello"));
    println!("{:?}", words_starting_with("the cat sat on the mat", 't'));
    println!("{:?}", pair_with_length(&["hi", "hello", "hey"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_chars() {
        let result = unique_chars("aabbcc");
        assert_eq!(result.len(), 3);
        assert!(result.contains(&'a'));
    }

    #[test]
    fn test_words_starting_with() {
        let result = words_starting_with("the cat sat on the mat", 't');
        let mut sorted = result.clone();
        sorted.sort();
        assert_eq!(sorted, vec!["the", "the"]);
    }

    #[test]
    fn test_pair_with_length() {
        let result = pair_with_length(&["hi", "hello"]);
        assert_eq!(result, vec![("hi", 2), ("hello", 5)]);
    }
}
