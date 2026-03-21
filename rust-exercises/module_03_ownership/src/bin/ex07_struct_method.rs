// EXERCISE 7: Fix the method that consumes its receiver.
// `fn method(self)` takes ownership of the struct — after calling it, the struct is gone.
// `fn method(&self)` borrows the struct — the caller can still use it after.
// Fix `print_title` so it borrows instead of consuming.
// Run: cargo test --bin ex07_struct_method -p module_03_ownership

struct Report {
    title: String,
    content: String,
}

impl Report {
    fn new(title: &str, content: &str) -> Report {
        Report {
            title: title.to_string(),
            content: content.to_string(),
        }
    }

    fn print_title(self) {          // BUG: takes ownership (self), should borrow (&self)
        println!("Title: {}", self.title);
    }

    fn summary(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

fn display(report: Report) {
    report.print_title();           // `report` is moved here
    println!("{}", report.summary()); // BUG: `report` was already moved
}

fn main() {
    let r = Report::new("Quarterly Results", "Revenue up 10%");
    display(r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report() {
        let r = Report::new("Test", "Content here");
        r.print_title();  // should borrow
        // r should still be usable after print_title
        assert_eq!(r.summary(), "Test: Content here");
    }
}
