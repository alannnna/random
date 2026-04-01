// EXERCISE 1: Add a lifetime annotation so the compiler knows where the output comes from.
// When a function returns a reference, the compiler needs to know which input it's
// borrowing from. Here, the output is either `a` or `b` — you must say they share
// a lifetime `'a` so the compiler can verify the output doesn't outlive its source.
// Run: cargo test --bin ex01_two_refs -p module_09_lifetimes

// returned reference comes from.
fn longer(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}

fn shorter(a: &str, b: &str) -> &str {
    if a.len() <= b.len() { a } else { b }
}

fn main() {
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xy");
        result = longer(s1.as_str(), s2.as_str());
        println!("Longer: {}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longer() {
        assert_eq!(longer("hello", "hi"), "hello");
        assert_eq!(longer("hi", "hello"), "hello");
        assert_eq!(longer("ab", "ab"), "ab");
    }

    #[test]
    fn test_shorter() {
        assert_eq!(shorter("hello", "hi"), "hi");
        assert_eq!(shorter("a", "bbb"), "a");
    }
}
