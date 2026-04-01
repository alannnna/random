// EXERCISE 2: Define a custom error type.
// Using String as an error type works, but a proper error enum lets callers
// match on specific failure modes. Define a `ParseError` enum and use it.
// Run: cargo test --bin ex02_custom_error -p module_08_robust

use std::fmt;

// Define it with these variants:
//   NotANumber(String)   — input couldn't be parsed as a number
//   OutOfRange(f64, f64, f64)  — value, min, max

// Implement it so errors print like:
//   "not a number: 'abc'"
//   "42 is out of range [0, 10]"

fn parse_bounded(s: &str, min: f64, max: f64) -> Result<f64, ParseError> {
    let n: f64 = s.parse().map_err(|_| ParseError::NotANumber(s.to_string()))?;
    if n < min || n > max {
        return Err(ParseError::OutOfRange(n, min, max));
    }
    Ok(n)
}

fn main() {
    println!("{:?}", parse_bounded("5", 0.0, 10.0));
    println!("{:?}", parse_bounded("abc", 0.0, 10.0));
    println!("{:?}", parse_bounded("42", 0.0, 10.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(parse_bounded("7.5", 0.0, 10.0), Ok(7.5));
    }

    #[test]
    fn test_not_a_number() {
        let e = parse_bounded("abc", 0.0, 10.0).unwrap_err();
        assert_eq!(format!("{}", e), "not a number: 'abc'");
    }

    #[test]
    fn test_out_of_range() {
        let e = parse_bounded("42", 0.0, 10.0).unwrap_err();
        assert_eq!(format!("{}", e), "42 is out of range [0, 10]");
    }
}
