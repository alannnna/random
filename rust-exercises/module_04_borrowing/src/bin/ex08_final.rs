// EXERCISE 8: The deliverable — a text formatter that borrows all the way through.
// Fix all the type signatures so they take &str instead of &String,
// and &[&str] where appropriate. No .clone() calls should be needed.
// Run: cargo test --bin ex08_final -p module_04_borrowing

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        s
    } else {
        // Find the last char boundary at or before max bytes
        let mut end = max;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        &s[..end]
    }
}

fn format_entry(name: &String, description: &String, max_desc: usize) -> String {  // BUG: should be &str
    let name_cap = capitalize_first(name);
    let short_desc = truncate(description, max_desc);
    format!("{}: {}", name_cap, short_desc)
}

fn format_all(entries: &Vec<(String, String)>, max_desc: usize) -> Vec<String> {  // BUG: should be &[(String, String)]
    entries
        .iter()
        .map(|(name, desc)| format_entry(name, desc, max_desc))
        .collect()
}

fn main() {
    let data = vec![
        (String::from("alice"), String::from("loves rust and borrowing")),
        (String::from("bob"), String::from("prefers python")),
    ];
    for line in format_all(&data, 14) {
        println!("{}", line);
    }

    // After fix, these should also work (passing &str literals directly):
    println!("{}", format_entry("carol", "writes haskell", 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_entry_str_literal() {
        // This only compiles once format_entry takes &str
        assert_eq!(format_entry("alice", "loves rust", 9), "Alice: loves rus");
        assert_eq!(format_entry("bob", "python", 100), "Bob: python");
    }

    #[test]
    fn test_format_all() {
        let data = vec![
            (String::from("alice"), String::from("loves rust")),
            (String::from("bob"), String::from("python")),
        ];
        let result = format_all(&data, 100);
        assert_eq!(result, vec!["Alice: loves rust", "Bob: python"]);
    }

    #[test]
    fn test_format_all_slice() {
        // After fix, &[] (array slice) should also work
        let data = [(String::from("carol"), String::from("haskell"))];
        let result = format_all(&data, 100);
        assert_eq!(result, vec!["Carol: haskell"]);
    }
}
