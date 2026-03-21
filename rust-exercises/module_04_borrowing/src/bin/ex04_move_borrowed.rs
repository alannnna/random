// EXERCISE 4: Fix the move out of a borrowed value.
// When you have `v: &Vec<String>`, indexing gives you `&String` — a reference, not the String.
// You can't move a value out from behind a reference (that would leave the ref dangling).
// Fix `take_first` to return a clone instead, or change the return type to &String.
// Run: cargo test --bin ex04_move_borrowed -p module_04_borrowing

fn take_first(v: &Vec<String>) -> String {
    v[0]   // BUG: cannot move out of `v[0]` which is behind a shared reference
}

fn take_last(v: &Vec<String>) -> String {
    let last_idx = v.len() - 1;
    v[last_idx]   // BUG: same problem
}

fn main() {
    let words = vec![
        String::from("hello"),
        String::from("world"),
        String::from("rust"),
    ];
    println!("First: {}", take_first(&words));
    println!("Last:  {}", take_last(&words));
    println!("Still have: {:?}", words);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_first() {
        let words = vec![String::from("hello"), String::from("world")];
        assert_eq!(take_first(&words), "hello");
        assert_eq!(words.len(), 2); // words should still be intact
    }

    #[test]
    fn test_take_last() {
        let words = vec![String::from("hello"), String::from("world")];
        assert_eq!(take_last(&words), "world");
        assert_eq!(words.len(), 2);
    }
}
