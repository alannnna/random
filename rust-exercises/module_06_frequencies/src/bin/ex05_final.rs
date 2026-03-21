// EXERCISE 5: The deliverable — a word frequency analyzer.
// Fix the three bugs so the analyzer correctly finds the top N words.
// Run: cargo test --bin ex05_final -p module_06_frequencies

use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        // Strip basic punctuation and lowercase
        let clean: String = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>()
            .to_lowercase();
        if !clean.is_empty() {
            *counts.entry(clean).or_insert(0) += 1;
        }
    }
    counts
}

fn top_n(counts: &HashMap<String, usize>, n: usize) -> Vec<(&str, usize)> {
    let mut pairs: Vec<(&str, usize)> = counts
        .iter()
        .map(|(k, &v)| (k.as_str(), v))
        .collect();

    pairs.sort_by_key(|&(word, count)| (count, word));  // BUG: should sort descending by count
                                                         // Hint: sort_by with .cmp().reverse()
    pairs.truncate(n);
    pairs
}

fn format_report(text: &str, n: usize) -> String {
    let counts = count_words(text);
    let top = top_n(&counts, n);
    top.iter()
        .map(|(word, count)| format!("{}: {}", count, word))  // BUG: word and count are swapped
        .collect::<Vec<_>>()
        .join(", ")
}

fn main() {
    let text = "the cat sat on the mat the cat sat on the";
    println!("{}", format_report(text, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        let counts = count_words("The the THE cat");
        assert_eq!(counts["the"], 3);
        assert_eq!(counts["cat"], 1);
    }

    #[test]
    fn test_top_n() {
        let counts = count_words("a a a b b c");
        let top = top_n(&counts, 2);
        assert_eq!(top[0], ("a", 3));
        assert_eq!(top[1], ("b", 2));
    }

    #[test]
    fn test_format_report() {
        let text = "the the the cat cat dog";
        let report = format_report(text, 3);
        // Should be "the: 3, cat: 2, dog: 1"
        assert!(report.starts_with("the: 3"), "got: {}", report);
    }
}
