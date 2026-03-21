// EXERCISE 5: Taking &mut instead of full ownership.
// `append_exclamation` modifies a String but doesn't need to own it.
// Change `make_excited` to borrow `text` mutably (&mut String) instead of taking it.
// The caller should still own the String afterwards.
// Run: cargo test --bin ex05_mut_ownership -p module_03_ownership

fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn make_excited(text: String) -> String {  // BUG: takes ownership unnecessarily
    let mut t = text;
    append_exclamation(&mut t);
    t
}

// Once you fix make_excited to take &mut String, callers can do this:
fn excite_all(messages: &mut Vec<String>) {
    for msg in messages.iter_mut() {
        make_excited(msg);  // BUG: this won't work until make_excited takes &mut String
    }
}

fn main() {
    let mut msgs = vec![
        String::from("Hello"),
        String::from("Goodbye"),
    ];
    excite_all(&mut msgs);
    println!("{:?}", msgs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_excited() {
        let mut s = String::from("Hello");
        make_excited(&mut s);
        assert_eq!(s, "Hello!");
    }

    #[test]
    fn test_excite_all() {
        let mut msgs = vec![String::from("Hi"), String::from("Bye")];
        excite_all(&mut msgs);
        assert_eq!(msgs, vec!["Hi!", "Bye!"]);
    }
}
