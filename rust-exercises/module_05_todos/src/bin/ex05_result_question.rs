// EXERCISE 5: The deliverable — a todo app that handles errors with Result and ?.
// `parse_id` can fail (non-numeric input). `execute` should propagate that failure
// using `?` instead of panicking. Fix the two marked bugs.
// Run: cargo test --bin ex05_result_question -p module_05_todos

#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Debug, PartialEq)]
enum AppError {
    InvalidId(String),
    NotFound(u32),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::InvalidId(s) => write!(f, "Invalid id: '{}'", s),
            AppError::NotFound(id) => write!(f, "Todo #{} not found", id),
        }
    }
}

fn parse_id(s: &str) -> Result<u32, AppError> {
    s.parse::<u32>().map_err(|_| AppError::InvalidId(s.to_string()))
}

fn find_mut<'a>(todos: &'a mut Vec<Todo>, id: u32) -> Result<&'a mut Todo, AppError> {
    todos.iter_mut().find(|t| t.id == id).ok_or(AppError::NotFound(id))
}

fn execute(todos: &mut Vec<Todo>, cmd: &str, arg: &str) -> Result<String, AppError> {
    match cmd {
        "done" => {
            let id = parse_id(arg);   // BUG: should use `?` to propagate error
            let todo = find_mut(todos, id)?;
            todo.done = true;
            Ok(format!("Marked '{}' as done", todo.title))
        }
        "add" => {
            let new_id = todos.len() as u32 + 1;
            todos.push(Todo { id: new_id, title: arg.to_string(), done: false });
            Ok(format!("Added '{}' with id {}", arg, new_id))
        }
        _ => Ok(format!("Unknown command: {}", cmd)),  // BUG: should return Err, not Ok
    }
}

fn main() {
    let mut todos = vec![
        Todo { id: 1, title: "Learn Rust".into(), done: false },
    ];
    match execute(&mut todos, "done", "1") {
        Ok(msg) => println!("OK: {}", msg),
        Err(e)  => println!("Error: {}", e),
    }
    match execute(&mut todos, "done", "abc") {
        Ok(msg) => println!("OK: {}", msg),
        Err(e)  => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_done_ok() {
        let mut todos = vec![Todo { id: 1, title: "Learn".into(), done: false }];
        let r = execute(&mut todos, "done", "1");
        assert_eq!(r, Ok("Marked 'Learn' as done".to_string()));
        assert!(todos[0].done);
    }

    #[test]
    fn test_done_invalid_id() {
        let mut todos = vec![Todo { id: 1, title: "Learn".into(), done: false }];
        let r = execute(&mut todos, "done", "abc");
        assert_eq!(r, Err(AppError::InvalidId("abc".to_string())));
    }

    #[test]
    fn test_unknown_command() {
        let mut todos = vec![];
        let r = execute(&mut todos, "frobnicate", "");
        assert!(r.is_err(), "unknown commands should return Err, not Ok");
    }
}
