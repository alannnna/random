// EXERCISE 5: The deliverable — a working greeter.
// The program compiles but produces wrong output. Fix the two logic bugs.
// Run: cargo test --bin ex05_final -p module_01_greeter

fn count_letters(name: &str) -> usize {
    name.chars().filter(|c| c.is_alphabetic()).count()
}

fn birth_year(age: u32) -> u32 {
    2026 - age
}

fn greet(name: &str, age: u32) -> String {
    let letters = count_letters(name);
    let year = birth_year(age);
    format!(
        "Hello, {}! Your name has {} letters and you were born in {}.",
        name, letters, year
    )
}

fn main() {
    println!("{}", greet("Alice", 30));
    println!("{}", greet("Bob Smith", 25));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(
            greet("Alice", 30),
            "Hello, Alice! Your name has 5 letters and you were born in 1996."
        );
        assert_eq!(
            greet("Bob Smith", 25),
            "Hello, Bob Smith! Your name has 8 letters and you were born in 2001."
        );
    }
}
