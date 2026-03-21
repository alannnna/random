// EXERCISE 4: Handle a new error variant.
// A new error variant `EmptyInput` was added to `ParseError` but the match in
// `describe_error` doesn't handle it yet — the compiler will tell you exactly where.
// Add the missing arm.
// Run: cargo test --bin ex04_new_variant -p module_08_robust

#[derive(Debug, PartialEq)]
enum ParseError {
    NotANumber(String),
    OutOfRange { value: f64, min: f64, max: f64 },
    EmptyInput,   // NEW — this variant is not yet handled below
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::NotANumber(s)                    => write!(f, "not a number: '{}'", s),
            ParseError::OutOfRange { value, min, max }   => write!(f, "{} out of range [{}, {}]", value, min, max),
            ParseError::EmptyInput                       => write!(f, "input was empty"),
        }
    }
}

fn describe_error(e: &ParseError) -> &'static str {
    match e {
        ParseError::NotANumber(_)   => "parse failure",
        ParseError::OutOfRange { .. } => "validation failure",
        // BUG: EmptyInput variant is not handled — non-exhaustive match
    }
}

fn parse(s: &str) -> Result<f64, ParseError> {
    if s.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    let n: f64 = s.parse().map_err(|_| ParseError::NotANumber(s.to_string()))?;
    if n < 0.0 || n > 100.0 {
        return Err(ParseError::OutOfRange { value: n, min: 0.0, max: 100.0 });
    }
    Ok(n)
}

fn main() {
    for input in &["50", "abc", "200", ""] {
        match parse(input) {
            Ok(n)  => println!("OK: {}", n),
            Err(e) => println!("Error ({}): {}", describe_error(&e), e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_all_variants() {
        assert_eq!(describe_error(&ParseError::NotANumber("x".into())), "parse failure");
        assert_eq!(describe_error(&ParseError::OutOfRange { value: 1.0, min: 0.0, max: 0.5 }), "validation failure");
        assert_eq!(describe_error(&ParseError::EmptyInput), "missing input");
    }

    #[test]
    fn test_parse_empty() {
        assert_eq!(parse(""), Err(ParseError::EmptyInput));
    }
}
