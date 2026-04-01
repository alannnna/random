// EXERCISE 6: Borrowing a slice of a slice — sub-borrows.
// When you take a sub-slice of a &str, the result borrows from the original data,
// not from the intermediate &str variable. Lifetimes must reflect this.
// Fix the struct and functions so sub-borrows compile and are correctly scoped.
// Run: cargo test --bin ex06_sub_borrow -p module_10_zero_copy

#[derive(Debug, PartialEq)]
struct Span {
    head: &str,  // compile error
    tail: &str,  // compile error
}

fn split_once_at(input: &str, delim: char) -> Option<Span> {
    input.find(delim).map(|i| Span {
        head: &input[..i],
        tail: &input[i + delim.len_utf8()..],
    })
}

// into one of them — must be explicit about which lifetime the output has
fn longer_head<'a>(a: &'a Span, b: &'a Span) -> &'a str {
    if a.head.len() >= b.head.len() { a.head } else { b.head }
}

fn main() {
    let input = String::from("first:second");
    if let Some(span) = split_once_at(&input, ':') {
        println!("head='{}' tail='{}'", span.head, span.tail);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_once() {
        let input = String::from("key=value");
        let span = split_once_at(&input, '=').unwrap();
        assert_eq!(span.head, "key");
        assert_eq!(span.tail, "value");
    }

    #[test]
    fn test_split_none() {
        let input = String::from("no_delimiter");
        assert_eq!(split_once_at(&input, '='), None);
    }

    #[test]
    fn test_longer_head() {
        let i1 = String::from("longer:x");
        let i2 = String::from("ab:yyy");
        let s1 = split_once_at(&i1, ':').unwrap();
        let s2 = split_once_at(&i2, ':').unwrap();
        assert_eq!(longer_head(&s1, &s2), "longer");
    }
}
