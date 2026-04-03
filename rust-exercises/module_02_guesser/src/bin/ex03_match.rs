// EXERCISE 3: Fix the non-exhaustive match.
// Rust's `match` must cover every possible value. If a pattern is missing,
// it's a compile error — the compiler won't let you forget a case.
// Run: cargo test --bin ex03_match -p module_02_guesser

#[derive(Debug)]
enum GuessResult {
    TooLow,
    TooHigh,
    Correct,
}

fn describe(result: GuessResult) -> &'static str {
    match result {
        GuessResult::TooLow  => "Go higher!",
        GuessResult::TooHigh => "Go lower!",
        GuessResult::Correct => "You got it!",
    }
}

fn main() {
    println!("{}", describe(GuessResult::TooLow));
    println!("{}", describe(GuessResult::TooHigh));
    println!("{}", describe(GuessResult::Correct));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe() {
        assert_eq!(describe(GuessResult::TooLow), "Go higher!");
        assert_eq!(describe(GuessResult::TooHigh), "Go lower!");
        assert_eq!(describe(GuessResult::Correct), "You got it!");
    }
}
