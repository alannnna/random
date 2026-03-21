// EXERCISE 4: Add a lifetime to a struct that holds a reference.
// When a struct holds a `&str` (or any reference), the struct itself needs a lifetime
// parameter to say "this struct cannot outlive the data it references."
// Fix the struct definition and its impl block.
// Run: cargo test --bin ex04_struct_ref -p module_09_lifetimes

// BUG: Excerpt holds a &str but has no lifetime parameter
#[derive(Debug)]
struct Excerpt {
    text: &str,    // compile error: missing lifetime specifier
    source: &str,  // compile error: same
}

impl Excerpt {
    fn new(text: &str, source: &str) -> Excerpt {
        Excerpt { text, source }
    }

    fn preview(&self) -> &str {
        if self.text.len() > 20 {
            &self.text[..20]
        } else {
            self.text
        }
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let source = String::from("Moby Dick");
        let excerpt = Excerpt::new(&novel[..16], &source);
        first_sentence = excerpt.preview();
        println!("Excerpt from {}: '{}'", excerpt.source, first_sentence);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excerpt() {
        let text = String::from("the quick brown fox jumps over the lazy dog");
        let source = String::from("a pangram");
        let e = Excerpt::new(&text, &source);
        assert_eq!(e.preview(), "the quick brown fox ");
        assert_eq!(e.source, "a pangram");
    }

    #[test]
    fn test_short_text() {
        let text = String::from("hi");
        let source = String::from("test");
        let e = Excerpt::new(&text, &source);
        assert_eq!(e.preview(), "hi");
    }
}
