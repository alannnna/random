// EXERCISE 1: Replace panics with Result.
// `panic!` crashes the whole program — it's appropriate for bugs, not bad input.
// For user-facing errors, return `Result<T, E>`: `Ok(value)` on success, `Err(e)` on failure.
// Rewrite `parse_age` and `parse_score` to return Result instead of panicking.
// Run: cargo test --bin ex01_panic_to_result -p module_08_robust

fn parse_age(s: &str) -> u32 {
    let n: i32 = s.parse().expect("age must be a number");
    if n < 0 || n > 150 {
        panic!("age {} is out of range", n);
    }
    n as u32
}

fn parse_score(s: &str) -> f64 {
    let n: f64 = s.parse().expect("score must be a number");
    if n < 0.0 || n > 100.0 {
        panic!("score {} out of range [0, 100]", n);
    }
    n
}

fn main() {
    // These should print errors, not crash:
    match parse_age("abc") {
        Ok(age) => println!("Age: {}", age),
        Err(e)  => println!("Error: {}", e),
    }
    match parse_score("200") {
        Ok(s) => println!("Score: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_age_ok() {
        assert_eq!(parse_age("25"), Ok(25));
    }

    #[test]
    fn test_parse_age_not_a_number() {
        assert!(parse_age("abc").is_err());
    }

    #[test]
    fn test_parse_age_out_of_range() {
        assert!(parse_age("200").is_err());
        assert!(parse_age("-5").is_err());
    }

    #[test]
    fn test_parse_score_ok() {
        assert_eq!(parse_score("85.5"), Ok(85.5));
    }

    #[test]
    fn test_parse_score_err() {
        assert!(parse_score("abc").is_err());
        assert!(parse_score("101").is_err());
    }
}
