// EXERCISE 2: Fix the word counter using the entry API.
// The naive approach (get then insert) requires two lookups and can have borrow issues.
// The idiomatic Rust way: `map.entry(key).or_insert(0)` gets-or-creates in one step.
// Fix `count_words` to produce correct counts using the entry API.
// Run: cargo test --bin ex02_hashmap_entry -p module_06_frequencies

use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for word in text.split_whitespace() {
        let w = word.to_lowercase();
        counts.insert(w, 1);
    }
    counts
}

fn most_frequent(counts: &HashMap<String, u32>) -> Option<(&str, u32)> {
    counts
        .iter()
        .max_by_key(|(_, &count)| count)
        .map(|(word, &count)| (word.as_str(), count))
}

fn main() {
    let text = "the cat sat on the mat the cat";
    let counts = count_words(text);
    println!("{:?}", counts);
    if let Some((word, count)) = most_frequent(&counts) {
        println!("Most frequent: '{}' ({} times)", word, count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        let counts = count_words("the cat sat on the mat the cat");
        assert_eq!(counts.get("the"), Some(&3));
        assert_eq!(counts.get("cat"), Some(&2));
        assert_eq!(counts.get("sat"), Some(&1));
    }

    #[test]
    fn test_most_frequent() {
        let counts = count_words("a a a b b c");
        let (word, count) = most_frequent(&counts).unwrap();
        assert_eq!(word, "a");
        assert_eq!(count, 3);
    }
}
