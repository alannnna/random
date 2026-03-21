// EXERCISE 6: Return lifetime tied to self vs tied to an argument.
// When a method takes &self and an argument, and returns a reference,
// the compiler needs to know: does the output borrow from `self`, or from the arg?
// Getting this wrong causes the caller to be incorrectly constrained.
// Fix the two methods so their output lifetimes are tied to the right input.
// Run: cargo test --bin ex06_self_vs_arg -p module_09_lifetimes

struct Cache {
    data: String,
}

impl Cache {
    fn new(data: &str) -> Cache {
        Cache { data: data.to_string() }
    }

    // This method returns a slice of self.data — output should be tied to 'self, not to 'query.
    // BUG: with elision, Rust ties output to `self` (first arg with a lifetime), which is
    // actually correct here. Try the other method below which IS broken.
    fn lookup(&self, _query: &str) -> &str {
        &self.data
    }

    // This method returns `fallback` if data is empty, otherwise returns a slice of self.data.
    // The output must be tied to BOTH `self` AND `fallback` — the same lifetime.
    // BUG: With elision, Rust ties the output only to `self`. But when we return `fallback`,
    // the output has `fallback`'s lifetime. Add an explicit lifetime to tie them together.
    fn get_or<'a>(&'a self, fallback: &str) -> &'a str {
        if self.data.is_empty() {
            fallback  // BUG: fallback has a different (unnamed) lifetime — compile error
        } else {
            &self.data
        }
    }
}

fn main() {
    let cache = Cache::new("cached value");

    let result = cache.lookup("anything");
    println!("lookup: {}", result);

    let fallback = String::from("default");
    let value = cache.get_or(&fallback);
    println!("get_or: {}", value);

    let empty_cache = Cache::new("");
    let fb = String::from("fallback data");
    let v = empty_cache.get_or(&fb);
    println!("empty get_or: {}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        let cache = Cache::new("hello");
        assert_eq!(cache.lookup("key"), "hello");
    }

    #[test]
    fn test_get_or_nonempty() {
        let cache = Cache::new("real data");
        let fb = String::from("fallback");
        assert_eq!(cache.get_or(&fb), "real data");
    }

    #[test]
    fn test_get_or_empty() {
        let cache = Cache::new("");
        let fb = String::from("fallback");
        assert_eq!(cache.get_or(&fb), "fallback");
    }
}
