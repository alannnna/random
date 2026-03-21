// EXERCISE 4: Add the missing return type.
// Rust functions that return a value must declare what type they return with `-> Type`.
// Without it, Rust assumes the function returns `()` (nothing), which conflicts with
// the value the function body is producing.
// Run: cargo test --bin ex04_functions -p module_01_greeter

fn count_letters(name: &str) {  // BUG: missing `-> usize`
    name.chars().filter(|c| c.is_alphabetic()).count()
}

fn main() {
    println!("Letters in 'Alice Smith': {}", count_letters("Alice Smith"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_letters() {
        assert_eq!(count_letters("Alice"), 5);
        assert_eq!(count_letters("Alice Smith"), 10);
        assert_eq!(count_letters("R2D2"), 2);
    }
}
