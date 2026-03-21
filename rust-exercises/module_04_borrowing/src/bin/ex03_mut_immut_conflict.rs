// EXERCISE 3: Fix the mutable/immutable borrow conflict.
// You cannot mutate a Vec while a reference into it is still live.
// If you push to a Vec, it might reallocate — which would invalidate any existing refs.
// Fix this by cloning `first` to get an owned String before mutating the Vec.
// Run: cargo test --bin ex03_mut_immut_conflict -p module_04_borrowing

fn duplicate_first(v: &mut Vec<String>) {
    let first = &v[0];          // immutable borrow of v — `first` points into v
    v.push(first.clone());      // BUG: mutable borrow of v while `first` (immutable) is still live
    println!("Added copy of: {}", first);  // `first` used here, keeps the borrow alive
}

fn main() {
    let mut words = vec![
        String::from("hello"),
        String::from("world"),
    ];
    duplicate_first(&mut words);
    println!("{:?}", words);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_first() {
        let mut words = vec![String::from("hello"), String::from("world")];
        duplicate_first(&mut words);
        assert_eq!(words, vec!["hello", "world", "hello"]);
    }
}
