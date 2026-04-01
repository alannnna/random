// EXERCISE 2: Fix the mutability error.
// In Rust, variables are immutable by default. You cannot reassign them unless
// you declare them with `mut`.
// Run: cargo test --bin ex02_mutability -p module_01_greeter

fn double_length(name: &str) -> usize {
    let mut len = name.len();
    len = len * 2;
    len
}

fn main() {
    println!("{}", double_length("Alice"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_length() {
        assert_eq!(double_length("Alice"), 10);
        assert_eq!(double_length("Hi"), 4);
    }
}
