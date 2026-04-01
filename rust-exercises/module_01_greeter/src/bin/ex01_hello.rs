// EXERCISE 1: Fix the format string.
// The format string uses `{}` as a placeholder but no value is provided to fill it.
// Run: cargo test --bin ex01_hello -p module_01_greeter

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", greet("world"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
        assert_eq!(greet("world"), "Hello, world!");
    }
}
