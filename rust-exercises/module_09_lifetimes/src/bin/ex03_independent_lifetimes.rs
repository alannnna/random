// EXERCISE 3: Two independent lifetimes.
// Sometimes a function takes two references that have nothing to do with each other,
// and returns something that only borrows from one of them.
// Using a single lifetime `'a` would incorrectly tie them together.
// Fix `prefix_of` to use two lifetime parameters.
// Run: cargo test --bin ex03_independent_lifetimes -p module_09_lifetimes

// BUG: `needle` and `haystack` are given the same lifetime 'a, but the output
// only borrows from `haystack`. This makes callers unnecessarily constrained.
fn find_prefix<'a>(haystack: &'a str, _needle: &'a str, n: usize) -> &'a str {
    &haystack[..n.min(haystack.len())]
}

// BUG: similar — `default` has nothing to do with the output when `opt` is Some
fn unwrap_or_prefix<'a>(opt: Option<&'a str>, _default: &'a str) -> &'a str {
    opt.unwrap_or("")
}

fn main() {
    let text = String::from("hello world");
    let prefix;
    {
        let needle = String::from("hello");
        prefix = find_prefix(&text, &needle, 5);
        // needle is dropped here — but prefix only borrows from text
    }
    println!("Prefix: {}", prefix);  // BUG: won't compile with one lifetime

    let result;
    {
        let default = String::from("fallback");
        let value = String::from("actual");
        result = unwrap_or_prefix(Some(&value), &default);
        println!("Result: {}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_prefix_outlives_needle() {
        let text = String::from("hello world");
        let prefix;
        {
            let needle = String::from("he");
            prefix = find_prefix(&text, &needle, 3);
        }
        assert_eq!(prefix, "hel");
    }

    #[test]
    fn test_unwrap_or_prefix_some() {
        let value = String::from("real value");
        let result;
        {
            let default = String::from("fallback");
            result = unwrap_or_prefix(Some(&value), &default);
        }
        assert_eq!(result, "real value");
    }
}
