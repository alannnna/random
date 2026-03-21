// EXERCISE 2: Tie the output lifetime to the correct input.
// When a function takes two references but only returns one of them,
// you can be precise about which input the output borrows from.
// Fix `first_word_of` so its output lifetime is tied only to `text`, not to `_sep`.
// This matters: it lets callers use the result even after `_sep` is dropped.
// Run: cargo test --bin ex02_explicit_lifetime -p module_09_lifetimes

// BUG: both parameters have the same lifetime 'a, so the output is constrained
// by the shorter-lived of the two. `_sep` outlives `text` in some call sites.
// Add a second lifetime 'b for `_sep` so the output only depends on `text`.
fn first_word_of<'a>(text: &'a str, _sep: &'a str) -> &'a str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let sentence = String::from("hello world foo");
    let first;
    {
        let sep = String::from(" ");
        // BUG: with one lifetime, `first` is constrained by sep's lifetime
        // After fix, `first` should be usable here even though sep is dropped
        first = first_word_of(&sentence, &sep);
    }
    println!("First word: {}", first);  // BUG: won't compile with single lifetime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        let sentence = String::from("hello world");
        let first;
        {
            let sep = String::from(" ");
            first = first_word_of(&sentence, &sep);
        }
        // `sep` is dropped here, but `first` should still be valid
        // because it only borrows from `sentence`
        assert_eq!(first, "hello");
    }
}
