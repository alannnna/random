// EXERCISE 2: Capture the call site, not the function site.
// `file!()` and `line!()` are built-in macros that expand to the file and line where they appear.
// Inside a regular function they always report *that function's* location — not the caller's.
// Replace `log_event` with a macro so `file!()` and `line!()` expand at the call site.
// Run: cargo test --bin ex02_source_location -p module_11_macros

// BUG: this function always reports "ex02_source_location.rs:13" no matter where it's called from
fn log_event(event: &str) -> String {
    format!("[{}:{}] {}", file!(), line!(), event)
}

// TODO: replace log_event with a macro so the location reflects the caller
// macro_rules! log_event {
//     ($event:expr) => { ... }
// }

fn main() {
    println!("{}", log_event("player_spawn"));
    println!("{}", log_event("enemy_death"));
}

#[cfg(test)]
mod tests {
    // Once log_event is a macro, these tests check that it captures *this* file
    // and the specific line the macro is called on.

    #[test]
    fn test_contains_filename() {
        let msg = log_event!("player_spawn");
        assert!(msg.contains("ex02_source_location.rs"), "expected filename in: {}", msg);
    }

    #[test]
    fn test_contains_event() {
        let msg = log_event!("enemy_death");
        assert!(msg.contains("enemy_death"), "expected event name in: {}", msg);
    }

    #[test]
    fn test_different_lines_differ() {
        let a = log_event!("first");
        let b = log_event!("second");
        // Each call is on a different line, so the line numbers should differ
        assert_ne!(a.split(':').nth(1), b.split(':').nth(1));
    }
}
