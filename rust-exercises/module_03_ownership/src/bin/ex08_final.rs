// EXERCISE 8: The deliverable — a text analysis pipeline with correct ownership.
// Fix all three functions to borrow their inputs instead of taking ownership.
// The pipeline should work without any .clone() calls.
// Run: cargo test --bin ex08_final -p module_03_ownership

fn most_common_char(text: String) -> char {   // BUG: should borrow
    let mut counts = [0u32; 128];
    for c in text.chars() {
        if (c as usize) < 128 && c != ' ' {
            counts[c as usize] += 1;
        }
    }
    counts
        .iter()
        .enumerate()
        .skip(33) // skip control chars
        .max_by_key(|(_, &count)| count)
        .map(|(i, _)| i as u8 as char)
        .unwrap_or('?')
}

fn truncate(text: String, max_len: usize) -> String {  // BUG: should borrow
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}

fn analyze(text: String) -> String {
    let words = text.split_whitespace().count();
    let common = most_common_char(text);   // `text` moved here
    let preview = truncate(text, 12);      // BUG: `text` already moved
    format!(
        "words={} most_common='{}' preview='{}'",
        words, common, preview
    )
}

fn main() {
    let passage = String::from("the cat sat on the mat");
    println!("{}", analyze(passage));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze() {
        let result = analyze(String::from("the cat sat on the mat"));
        assert!(result.contains("words=6"), "got: {}", result);
        assert!(result.contains("most_common='t'"), "got: {}", result);
        assert!(result.contains("preview='the cat sat '"), "got: {}", result);
    }
}
