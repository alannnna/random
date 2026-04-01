// EXERCISE 3: Fix the type mismatch.
// `birth_year` expects a u32, but `age` is a &str (a string slice).
// You need to parse the string into a number first.
// Hint: "42".parse::<u32>().unwrap() converts a &str to u32.
// Run: cargo test --bin ex03_types -p module_01_greeter

fn birth_year(age: u32) -> u32 {
    2025 - age
}

fn name_stats(name: &str, age: &str) -> String {
    let year = birth_year(age.parse().unwrap());
    format!("{} has {} letters and was born in {}", name, name.len(), year)
}

fn main() {
    println!("{}", name_stats("Alice", "30"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_stats() {
        assert_eq!(
            name_stats("Alice", "30"),
            "Alice has 5 letters and was born in 1995"
        );
        assert_eq!(
            name_stats("Bob", "25"),
            "Bob has 3 letters and was born in 2000"
        );
    }
}
