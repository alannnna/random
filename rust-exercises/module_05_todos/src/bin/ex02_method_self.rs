// EXERCISE 2: Fix the method receivers.
// `complete` needs to mutate `self.done`, so it requires `&mut self`.
// `summary` only reads fields, so `&self` is correct.
// Run: cargo test --bin ex02_method_self -p module_05_todos

#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

impl Todo {
    fn new(id: u32, title: &str) -> Todo {
        Todo { id, title: title.to_string(), done: false }
    }

    fn complete(self) {
        self.done = true;
    }

    fn summary(&self) -> String {
        let status = if self.done { "✓" } else { "○" };
        format!("[{}] {} (id={})", status, self.title, self.id)
    }
}

fn main() {
    let mut t = Todo::new(1, "Learn Rust");
    t.complete();
    println!("{}", t.summary());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete() {
        let mut t = Todo::new(1, "Write code");
        assert!(!t.done);
        t.complete();
        assert!(t.done);
        // After fix, t should still be accessible here
        assert_eq!(t.summary(), "[✓] Write code (id=1)");
    }

    #[test]
    fn test_summary_not_done() {
        let t = Todo::new(2, "Read docs");
        assert_eq!(t.summary(), "[○] Read docs (id=2)");
    }
}
