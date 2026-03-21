// EXERCISE 3: Fix the non-exhaustive enum match.
// The `Command` enum has 4 variants. The `describe` function only handles 2.
// Rust requires every variant to be covered — add the missing arms.
// Run: cargo test --bin ex03_enum_match -p module_05_todos

#[derive(Debug)]
enum Command {
    Add(String),
    Done(u32),
    List,
    Quit,
}

fn describe(cmd: &Command) -> String {
    match cmd {
        Command::Add(title) => format!("Adding: '{}'", title),
        Command::Done(id)   => format!("Marking #{} as done", id),
        // BUG: List and Quit variants are not handled
    }
}

fn main() {
    let commands = vec![
        Command::Add("Buy groceries".to_string()),
        Command::Done(1),
        Command::List,
        Command::Quit,
    ];
    for cmd in &commands {
        println!("{}", describe(cmd));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe() {
        assert_eq!(describe(&Command::Add("hello".into())), "Adding: 'hello'");
        assert_eq!(describe(&Command::Done(3)), "Marking #3 as done");
        assert_eq!(describe(&Command::List), "Listing all todos");
        assert_eq!(describe(&Command::Quit), "Goodbye!");
    }
}
