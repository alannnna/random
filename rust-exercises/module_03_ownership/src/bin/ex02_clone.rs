// EXERCISE 2: Fix the move error using .clone().
// Sometimes you genuinely need two owned copies of a value.
// `.clone()` creates a deep copy — both the original and the clone are independently owned.
// Fix `greet_loudly` so it can use `name` after passing it to `loudify`.
// Run: cargo test --bin ex02_clone -p module_03_ownership

fn loudify(s: String) -> String {
    s.to_uppercase()
}

fn greet_loudly(name: String) -> (String, String) {
    let loud = loudify(name);
    (name, loud)
}

fn main() {
    let (original, loud) = greet_loudly(String::from("Alice"));
    println!("Original: {}", original);
    println!("Loud:     {}", loud);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_loudly() {
        let (original, loud) = greet_loudly(String::from("Alice"));
        assert_eq!(original, "Alice");
        assert_eq!(loud, "ALICE");
    }
}
