// EXERCISE 1: Fix the use-after-move error.
// When you pass a String to a function, ownership transfers (moves) into that function.
// After the move, the original variable is gone — you can't use it anymore.
// Fix `process` so it doesn't consume its input.
// Run: cargo test --bin ex01_use_after_move -p module_03_ownership

fn get_length(s: String) -> usize {
    s.len()
}

fn process(name: String) -> (usize, String) {
    let len = get_length(name);  // `name` is moved into get_length here
    (len, name)                  // BUG: `name` was already moved, can't use it here
}

fn main() {
    let (len, name) = process(String::from("Alice"));
    println!("{} has {} characters", name, len);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let (len, name) = process(String::from("Alice"));
        assert_eq!(len, 5);
        assert_eq!(name, "Alice");
    }
}
