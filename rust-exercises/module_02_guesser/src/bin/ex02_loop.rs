// EXERCISE 2: Fix the loop so it returns the right value.
// In Rust, `loop { ... break value; }` is an expression that produces a value.
// This function should find the first number >= start that is a multiple of 7.
// Run: cargo test --bin ex02_loop -p module_02_guesser

fn first_multiple_of_7(start: u32) -> u32 {
    let mut n = start;
    loop {
        if n % 7 == 0 {
            break n - 1;
        }
        n += 1;
    }
}

fn main() {
    println!("First multiple of 7 >= 1:  {}", first_multiple_of_7(1));
    println!("First multiple of 7 >= 10: {}", first_multiple_of_7(10));
    println!("First multiple of 7 >= 14: {}", first_multiple_of_7(14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_multiple_of_7() {
        assert_eq!(first_multiple_of_7(1), 7);
        assert_eq!(first_multiple_of_7(10), 14);
        assert_eq!(first_multiple_of_7(14), 14);
        assert_eq!(first_multiple_of_7(15), 21);
    }
}
