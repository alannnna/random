// EXERCISE 4: Handle None instead of unwrapping.
// `.unwrap()` panics if the value is `None`. That's a crash.
// Fix `find_todo` to return an `Option<&Todo>` and fix `print_todo`
// to handle the None case gracefully instead of panicking.
// Run: cargo test --bin ex04_option_unwrap -p module_05_todos

#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

fn find_todo<'a>(todos: &'a [Todo], id: u32) -> &'a Todo {
    todos.iter().find(|t| t.id == id).unwrap()  // BUG: panics if id not found
}

fn print_todo(todos: &[Todo], id: u32) -> String {
    let todo = find_todo(todos, id);  // crashes if missing
    format!("[{}] {}", if todo.done { "✓" } else { "○" }, todo.title)
}

fn main() {
    let todos = vec![
        Todo { id: 1, title: "Learn Rust".into(), done: false },
        Todo { id: 2, title: "Write tests".into(), done: true },
    ];
    println!("{}", print_todo(&todos, 1));
    println!("{}", print_todo(&todos, 99)); // crashes!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_existing() {
        let todos = vec![
            Todo { id: 1, title: "Learn Rust".into(), done: false },
        ];
        assert_eq!(print_todo(&todos, 1), "[○] Learn Rust");
    }

    #[test]
    fn test_print_missing() {
        let todos = vec![
            Todo { id: 1, title: "Learn Rust".into(), done: false },
        ];
        // Should return a "not found" message, not panic
        assert_eq!(print_todo(&todos, 99), "Not found: #99");
    }
}
