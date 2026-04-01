// EXERCISE 7: Knowing when to give up and own data.
// Zero-copy is great, but sometimes you can't borrow — the data you need doesn't
// exist yet (it must be computed). In those cases, return an owned String.
// Fix `normalize` so it returns an owned String when it can't just slice the input.
// Run: cargo test --bin ex07_owned_vs_borrowed -p module_10_zero_copy

// This function tries to return a &str from `input` — sometimes it can (if no
// normalization is needed), but when it lowercases, it must return a new String.
// Fix it to return String always (owned), or use Cow<str> for zero-copy when possible.
// For now, fix it by returning String.
fn normalize(input: &str) -> &str {
    let trimmed = input.trim();
    if trimmed.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()) {
        trimmed  // no change needed — could return a borrow
    } else {
        &trimmed.to_lowercase()  // compile error: returning reference to temporary
    }
}

// Once normalize returns String, fix this to use it:
fn normalize_all(inputs: &[&str]) -> Vec<String> {
    inputs.iter().map(|s| normalize(s).to_string()).collect()
}

fn main() {
    let words = vec!["  Hello  ", "WORLD", "already_fine", "  MiXeD  "];
    for norm in normalize_all(&words) {
        println!("'{}'", norm);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_lowercase() {
        assert_eq!(normalize("  Hello  "), "hello");
        assert_eq!(normalize("WORLD"), "world");
    }

    #[test]
    fn test_normalize_already_fine() {
        assert_eq!(normalize("  already_fine  "), "already_fine");
        assert_eq!(normalize("hello"), "hello");
    }

    #[test]
    fn test_normalize_all() {
        let result = normalize_all(&["Hello", "world", "RUST"]);
        assert_eq!(result, vec!["hello", "world", "rust"]);
    }
}
