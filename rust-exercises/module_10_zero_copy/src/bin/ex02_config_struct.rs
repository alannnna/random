// EXERCISE 2: Lifetimes through nested structs.
// When a struct contains another struct that has a lifetime, the outer struct
// must also declare and thread that lifetime through.
// Fix `Pair` and `Config` so they compile.
// Run: cargo test --bin ex02_config_struct -p module_10_zero_copy

// BUG: missing lifetime
#[derive(Debug, PartialEq)]
struct Pair {
    key: &str,    // compile error
    value: &str,  // compile error
}

// BUG: missing lifetime (and Pair inside needs a lifetime too)
#[derive(Debug)]
struct Config {
    pairs: Vec<Pair>,  // compile error: Pair needs a lifetime argument
}

impl Config {
    fn new() -> Config {
        Config { pairs: Vec::new() }
    }

    fn add(&mut self, key: &str, value: &str) {
        self.pairs.push(Pair { key, value });
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.pairs.iter().find(|p| p.key == key).map(|p| p.value)
    }

    fn len(&self) -> usize {
        self.pairs.len()
    }
}

fn parse_config(input: &str) -> Config {
    let mut config = Config::new();
    for line in input.lines() {
        if let Some(i) = line.find('=') {
            config.add(line[..i].trim(), line[i+1..].trim());
        }
    }
    config
}

fn main() {
    let input = String::from("host = localhost\nport = 8080\ndebug = true");
    let config = parse_config(&input);
    println!("host: {:?}", config.get("host"));
    println!("port: {:?}", config.get("port"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let input = String::from("host = localhost\nport = 8080");
        let config = parse_config(&input);
        assert_eq!(config.get("host"), Some("localhost"));
        assert_eq!(config.get("port"), Some("8080"));
        assert_eq!(config.get("missing"), None);
        assert_eq!(config.len(), 2);
    }
}
