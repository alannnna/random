// EXERCISE 1: Fix the dangling reference.
// This function creates a String locally and tries to return a reference to it.
// But the String is dropped when the function returns — the reference would point
// to freed memory. Rust won't compile this. Fix it by returning an owned String.
// Run: cargo test --bin ex01_dangling -p module_04_borrowing

fn make_greeting(name: &str) -> &str {   // BUG: can't return a reference to local data
    let s = format!("Hello, {}!", name);
    &s  // `s` is dropped here — this reference would be dangling
}

fn main() {
    let greeting = make_greeting("Alice");
    println!("{}", greeting);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_greeting() {
        assert_eq!(make_greeting("Alice"), "Hello, Alice!");
        assert_eq!(make_greeting("Bob"), "Hello, Bob!");
    }
}
