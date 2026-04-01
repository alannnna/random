// EXERCISE 1: Fix the comparison operator.
// `=` is assignment. `==` is equality comparison. In Rust, `if` requires
// a boolean expression — using `=` here is a compile error.
// Run: cargo test --bin ex01_comparison -p module_02_guesser

fn check_guess(guess: i32, secret: i32) -> &'static str {
    if guess == secret {
        "correct"
    } else if guess < secret {
        "too low"
    } else {
        "too high"
    }
}

fn main() {
    println!("{}", check_guess(5, 7));
    println!("{}", check_guess(9, 7));
    println!("{}", check_guess(7, 7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_guess() {
        assert_eq!(check_guess(5, 7), "too low");
        assert_eq!(check_guess(9, 7), "too high");
        assert_eq!(check_guess(7, 7), "correct");
    }
}
