// EXERCISE 4: Fix a lifetime that was inferred as 'static when it shouldn't be.
// If you write a struct with a `&'static str` field, Rust will only accept
// string literals — runtime strings won't compile.
// The struct should work with any lifetime, not just 'static.
// Run: cargo test --bin ex04_static_inferred -p module_10_zero_copy

#[derive(Debug)]
struct Token {
    kind: &'static str,   // BUG: 'static means only literals allowed
    value: &'static str,  // BUG: same
}

impl Token {
    fn new(kind: &'static str, value: &'static str) -> Token {
        Token { kind, value }
    }

    fn is_keyword(&self) -> bool {
        matches!(self.kind, "keyword")
    }

    fn describe(&self) -> String {
        format!("{}({})", self.kind, self.value)
    }
}

fn tokenize_first(input: &str) -> Token {
    let trimmed = input.trim();
    // BUG: `trimmed` is a slice of `input`, not a 'static string
    // This won't compile once we try to store it in Token
    if trimmed == "if" || trimmed == "let" || trimmed == "fn" {
        Token::new("keyword", trimmed)
    } else if trimmed.chars().all(|c| c.is_numeric()) {
        Token::new("number", trimmed)
    } else {
        Token::new("ident", trimmed)
    }
}

fn main() {
    let src = String::from("  let  ");
    let tok = tokenize_first(&src);
    println!("{:?} is_keyword={}", tok.describe(), tok.is_keyword());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword() {
        let src = String::from("if");
        let tok = tokenize_first(&src);
        assert_eq!(tok.kind, "keyword");
        assert_eq!(tok.value, "if");
        assert!(tok.is_keyword());
    }

    #[test]
    fn test_number() {
        let src = String::from("  42  ");
        let tok = tokenize_first(&src);
        assert_eq!(tok.kind, "number");
        assert_eq!(tok.value, "42");
        assert!(!tok.is_keyword());
    }

    #[test]
    fn test_ident() {
        let src = String::from("foo");
        let tok = tokenize_first(&src);
        assert_eq!(tok.describe(), "ident(foo)");
    }
}
