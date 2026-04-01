// EXERCISE 5: Lifetime annotations on impl blocks.
// When a struct has a lifetime parameter, the impl block must declare it too.
// The syntax is: impl<'a> MyStruct<'a> { ... }
// Fix the impl block below — it's missing the lifetime.
// Run: cargo test --bin ex05_impl_lifetime -p module_09_lifetimes

#[derive(Debug)]
struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl StrSplit {           // should be: impl<'a> StrSplit<'a>
    fn new(s: &str, delim: &str) -> StrSplit {
        StrSplit { remainder: s, delimiter: delim }
    }

    fn next_token(&mut self) -> Option<&str> {
        if self.remainder.is_empty() {
            return None;
        }
        match self.remainder.find(self.delimiter) {
            Some(i) => {
                let token = &self.remainder[..i];
                self.remainder = &self.remainder[i + self.delimiter.len()..];
                Some(token)
            }
            None => {
                let token = self.remainder;
                self.remainder = "";
                Some(token)
            }
        }
    }
}

fn split_all<'a>(s: &'a str, delim: &'a str) -> Vec<&'a str> {
    let mut splitter = StrSplit::new(s, delim);
    let mut result = Vec::new();
    while let Some(token) = splitter.next_token() {
        result.push(token);
    }
    result
}

fn main() {
    let data = String::from("one::two::three::four");
    let parts = split_all(&data, "::");
    println!("{:?}", parts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_all() {
        assert_eq!(split_all("a::b::c", "::"), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_single_token() {
        assert_eq!(split_all("hello", "::"), vec!["hello"]);
    }

    #[test]
    fn test_empty() {
        let result = split_all("", "::");
        assert_eq!(result, vec![""] as Vec<&str>);
    }
}
