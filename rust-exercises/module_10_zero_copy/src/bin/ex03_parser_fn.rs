// EXERCISE 3: Lifetime on a parser function's return type.
// A parser function takes `&str` input and returns a slice of that input.
// The output's lifetime must be tied to the input's lifetime — not just elided.
// Fix the two functions that have missing or wrong lifetime annotations.
// Run: cargo test --bin ex03_parser_fn -p module_10_zero_copy

fn parse_until(input: &str, stop: char) -> &str {
    match input.find(stop) {
        Some(i) => &input[..i],
        None    => input,
    }
}

fn parse_nonempty(primary: &str, fallback: &str) -> &str {
    if !primary.is_empty() { primary } else { fallback }
}

// This one is correct — leave it alone.
fn skip_whitespace(s: &str) -> &str {
    s.trim_start()
}

fn parse_key_value(input: &str) -> Option<(&str, &str)> {
    let input = skip_whitespace(input);
    let key = parse_until(input, '=');
    if key.len() >= input.len() {
        return None;
    }
    let rest = &input[key.len() + 1..];
    let value = skip_whitespace(rest);
    let key = key.trim_end();
    Some((key, value))
}

fn main() {
    let inputs = ["name = Alice", "  count=42", "no_equals"];
    for input in &inputs {
        match parse_key_value(input) {
            Some((k, v)) => println!("'{}' = '{}'", k, v),
            None         => println!("no key-value in '{}'", input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_until() {
        assert_eq!(parse_until("hello=world", '='), "hello");
        assert_eq!(parse_until("noeq", '='), "noeq");
    }

    #[test]
    fn test_parse_nonempty() {
        assert_eq!(parse_nonempty("hello", "world"), "hello");
        assert_eq!(parse_nonempty("", "fallback"), "fallback");
    }

    #[test]
    fn test_parse_key_value() {
        assert_eq!(parse_key_value("name = Alice"), Some(("name", "Alice")));
        assert_eq!(parse_key_value("count=42"), Some(("count", "42")));
        assert_eq!(parse_key_value("no_equals"), None);
    }
}
