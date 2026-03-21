// EXERCISE 8: The deliverable — a zero-copy config file parser.
// Parses "key = value" lines. All keys and values are slices of the original input.
// No String allocations except for the final report.
// Fix all lifetime annotations — there are several.
// Run: cargo test --bin ex08_final -p module_10_zero_copy

// BUG: Pair holds borrowed slices — needs a lifetime
#[derive(Debug, PartialEq)]
struct Pair {
    key: &str,    // compile error
    value: &str,  // compile error
}

// BUG: Config holds a Vec of borrowed Pairs — needs a lifetime too
#[derive(Debug)]
struct Config {
    pairs: Vec<Pair>,  // compile error: Pair needs a lifetime argument
}

// BUG: output lifetime is ambiguous (two input references)
impl Pair {
    fn new(key: &str, value: &str) -> Pair {
        Pair { key, value }
    }
}

// BUG: Config's impl needs the lifetime threaded through
impl Config {
    fn new() -> Config {
        Config { pairs: Vec::new() }
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.pairs.iter().find(|p| p.key == key).map(|p| p.value)
    }

    fn keys(&self) -> Vec<&str> {
        self.pairs.iter().map(|p| p.key).collect()
    }
}

// BUG: the returned Config borrows from `input` — lifetime annotation needed on fn
fn parse(input: &str) -> Config {
    let mut config = Config::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(eq) = line.find('=') {
            let key   = line[..eq].trim();
            let value = line[eq + 1..].trim();
            config.pairs.push(Pair::new(key, value));
        }
    }
    config
}

fn report(config: &Config) -> String {
    config
        .pairs
        .iter()
        .map(|p| format!("{} -> {}", p.key, p.value))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let input = String::from(
        "# Database config\n\
         host = localhost\n\
         port = 5432\n\
         name = mydb\n\
         \n\
         # App config\n\
         debug = true"
    );

    let config = parse(&input);
    println!("{}", report(&config));
    println!("\nhost = {:?}", config.get("host"));
    println!("keys: {:?}", config.keys());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        let input = String::from("host = localhost\nport = 5432");
        let config = parse(&input);
        assert_eq!(config.get("host"), Some("localhost"));
        assert_eq!(config.get("port"), Some("5432"));
        assert_eq!(config.get("missing"), None);
    }

    #[test]
    fn test_skips_comments_and_blanks() {
        let input = String::from("# comment\n\nkey = value\n# another");
        let config = parse(&input);
        assert_eq!(config.pairs.len(), 1);
        assert_eq!(config.get("key"), Some("value"));
    }

    #[test]
    fn test_keys() {
        let input = String::from("a = 1\nb = 2\nc = 3");
        let config = parse(&input);
        let mut keys = config.keys();
        keys.sort();
        assert_eq!(keys, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_zero_copy_pairs_borrow_input() {
        let input = String::from("name = alice");
        let config = parse(&input);
        // The key and value are slices of `input`, not new Strings
        let pair = &config.pairs[0];
        assert_eq!(pair.key, "name");
        assert_eq!(pair.value, "alice");
    }
}
