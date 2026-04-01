// EXERCISE 7: Understand lifetime elision — when annotations are implied.
// Rust has three elision rules that let you omit lifetimes in common cases.
// Rule 1: each input reference gets its own lifetime.
// Rule 2: if there's exactly one input lifetime, all outputs get it.
// Rule 3: if one input is &self or &mut self, all outputs get self's lifetime.
//
// Fix the functions where elision doesn't apply (compiler will tell you)
// and leave alone the ones where it does.
// Run: cargo test --bin ex07_elision -p module_09_lifetimes

// Elision works here (Rule 2: one input → output gets same lifetime). No annotation needed.
fn first_char(s: &str) -> &str {
    &s[..1.min(s.len())]
}

// Which input does the output come from? You must say explicitly.
fn pick_nonempty(a: &str, b: &str) -> &str {
    if !a.is_empty() { a } else { b }
}

// Elision works here (Rule 3: output tied to &self). No annotation needed.
struct Wrapper(String);
impl Wrapper {
    fn inner(&self) -> &str {
        &self.0
    }
}

// Elision would tie output to self, but that's wrong here.
impl Wrapper {
    fn or_default<'a>(&'a self, default: &str) -> &'a str {
        if self.0.is_empty() { default } else { &self.0 }
        //      but we return it as if it has lifetime 'a
    }
}

fn main() {
    let s = String::from("hello");
    println!("{}", first_char(&s));

    let a = String::from("");
    let result;
    {
        let b = String::from("world");
        result = pick_nonempty(&a, &b);
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_char() {
        assert_eq!(first_char("hello"), "h");
        assert_eq!(first_char(""), "");
    }

    #[test]
    fn test_pick_nonempty() {
        assert_eq!(pick_nonempty("hello", "world"), "hello");
        assert_eq!(pick_nonempty("", "world"), "world");
    }

    #[test]
    fn test_wrapper() {
        let w = Wrapper("data".to_string());
        assert_eq!(w.inner(), "data");
    }

    #[test]
    fn test_or_default() {
        let w = Wrapper("value".to_string());
        let d = String::from("default");
        assert_eq!(w.or_default(&d), "value");

        let empty = Wrapper("".to_string());
        let d2 = String::from("fallback");
        assert_eq!(empty.or_default(&d2), "fallback");
    }
}
