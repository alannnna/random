// EXERCISE 1: Add a lifetime to a struct that borrows its data.
// A zero-copy parser stores slices of the original input — it never allocates
// new Strings. But a struct holding a &str needs a lifetime parameter.
// Fix the `Key` struct so it compiles.
// Run: cargo test --bin ex01_key_struct -p module_10_zero_copy

#[derive(Debug, PartialEq)]
struct Key {
    name: &str,      // compile error: missing lifetime specifier
    namespace: &str, // compile error: same
}

impl Key {
    fn new(namespace: &str, name: &str) -> Key {
        Key { name, namespace }
    }

    fn full_name(&self) -> String {
        if self.namespace.is_empty() {
            self.name.to_string()
        } else {
            format!("{}.{}", self.namespace, self.name)
        }
    }
}

fn parse_key(s: &str) -> Key {
    match s.find('.') {
        Some(i) => Key::new(&s[..i], &s[i+1..]),
        None    => Key::new("", s),
    }
}

fn main() {
    let input = String::from("config.max_connections");
    let key = parse_key(&input);
    println!("{:?} -> full: {}", key, key.full_name());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_namespace() {
        let input = String::from("config.timeout");
        let key = parse_key(&input);
        assert_eq!(key.namespace, "config");
        assert_eq!(key.name, "timeout");
        assert_eq!(key.full_name(), "config.timeout");
    }

    #[test]
    fn test_parse_no_namespace() {
        let input = String::from("version");
        let key = parse_key(&input);
        assert_eq!(key.namespace, "");
        assert_eq!(key.name, "version");
        assert_eq!(key.full_name(), "version");
    }
}
