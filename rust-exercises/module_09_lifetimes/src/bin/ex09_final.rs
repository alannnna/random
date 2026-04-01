// EXERCISE 9: The deliverable — a string utilities library with correct lifetimes.
// Fix all the lifetime annotations. The functions borrow from their inputs and
// return references into them — zero heap allocation.
// Run: cargo test --bin ex09_final -p module_09_lifetimes

fn longest_word(text: &str, _tie_breaker: &str) -> &str {
    text.split_whitespace()
        .max_by_key(|w| w.len())
        .unwrap_or(_tie_breaker)
}

#[derive(Debug)]
struct TextStats {
    source: &str,
    longest: &str,
    shortest: &str,
}

impl TextStats {
    fn new(source: &str) -> TextStats {
        let words: Vec<&str> = source.split_whitespace().collect();
        let longest = words.iter().copied().max_by_key(|w| w.len()).unwrap_or("");
        let shortest = words.iter().copied().min_by_key(|w| w.len()).unwrap_or("");
        TextStats { source, longest, shortest }
    }

    fn summary(&self) -> String {
        format!(
            "words={} longest='{}' shortest='{}'",
            self.source.split_whitespace().count(),
            self.longest,
            self.shortest
        )
    }

    fn words_starting_with(&self, prefix: &str) -> Vec<&str> {
        self.source
            .split_whitespace()
            .filter(|w| w.starts_with(prefix))
            .collect()
    }
}

fn main() {
    let text = String::from("the quick brown fox jumps over the lazy dog");
    let fallback = String::from("?");
    let stats = TextStats::new(&text);
    println!("{}", stats.summary());
    println!("Longest word (fallback={}): {}", fallback, longest_word(&text, &fallback));
    println!("Words starting with 't': {:?}", stats.words_starting_with("t"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_word() {
        let text = String::from("the quick brown fox");
        let fb = String::from("?");
        assert_eq!(longest_word(&text, &fb), "quick");
    }

    #[test]
    fn test_longest_word_uses_fallback() {
        let text = String::from("");
        let fb = String::from("fallback");
        assert_eq!(longest_word(&text, &fb), "fallback");
    }

    #[test]
    fn test_stats_summary() {
        let text = String::from("the quick brown fox");
        let stats = TextStats::new(&text);
        assert_eq!(stats.summary(), "words=4 longest='quick' shortest='the'");
    }

    #[test]
    fn test_words_starting_with() {
        let text = String::from("the cat sat on the mat");
        let stats = TextStats::new(&text);
        let mut result = stats.words_starting_with("t");
        result.sort();
        assert_eq!(result, vec!["the", "the"]);
    }
}
