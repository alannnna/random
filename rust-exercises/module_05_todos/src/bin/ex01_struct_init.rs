// EXERCISE 1: Fix the struct initialization.
// Every field of a struct must be provided when constructing it.
// The `Todo` struct has 3 fields, but `new` only sets 2.
// Run: cargo test --bin ex01_struct_init -p module_05_todos

#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

impl Todo {
    fn new(id: u32, title: &str) -> Todo {
        Todo {
            id,
            title: title.to_string(),
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }
}

fn main() {
    let t = Todo::new(1, "Learn Rust");
    println!("{:?}", t);
    println!("Done: {}", t.is_done());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {
        let t = Todo::new(1, "Learn Rust");
        assert_eq!(t.id, 1);
        assert_eq!(t.title, "Learn Rust");
        assert!(!t.is_done());  // new todos should not be done
    }
}
