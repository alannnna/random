// EXERCISE 8: When 'static is wrong.
// `'static` means "lives for the entire program." String literals are `'static`.
// Requiring `'static` from callers who have shorter-lived data is usually wrong.
// Fix `log_message` and `Cache` to use a proper lifetime parameter instead of `'static`.
// Run: cargo test --bin ex08_static -p module_09_lifetimes

// BUG: the 'static bound means only string literals can be passed
fn log_message(msg: &'static str) -> String {
    format!("[LOG] {}", msg)
}

struct Cache {
    label: &'static str,   // BUG: forces label to be a string literal
    value: u32,
}

impl Cache {
    fn new(label: &'static str, value: u32) -> Cache {  // BUG: same
        Cache { label, value }
    }

    fn describe(&self) -> String {
        format!("{}: {}", self.label, self.value)
    }
}

fn main() {
    // These work (literals are 'static):
    println!("{}", log_message("startup complete"));

    // BUG: this doesn't work — dynamic strings aren't 'static:
    let dynamic = format!("user {} logged in", "alice");
    println!("{}", log_message(&dynamic));

    let label = String::from("my cache");
    let cache = Cache::new(&label, 42);  // BUG: label isn't 'static
    println!("{}", cache.describe());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_literal() {
        assert_eq!(log_message("hello"), "[LOG] hello");
    }

    #[test]
    fn test_log_dynamic() {
        // After fix, dynamic strings should work too
        let msg = format!("event {}", 42);
        assert_eq!(log_message(&msg), "[LOG] event 42");
    }

    #[test]
    fn test_cache_dynamic_label() {
        let label = String::from("results");
        let cache = Cache::new(&label, 100);
        assert_eq!(cache.describe(), "results: 100");
    }
}
