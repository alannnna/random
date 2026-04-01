// EXERCISE 1: Fix the macro call syntax.
// Macros look like functions but are called with `!` after the name: `println!`, `vec!`, `format!`.
// Without `!`, the compiler looks for a *function* with that name and won't find one.
// Fix the two broken calls below.
// Run: cargo test --bin ex01_macro_syntax -p module_11_macros

fn main() {
    // BUG: `vec` is a macro, not a function — needs `!`
    let events: Vec<&str> = vec("player_spawn", "enemy_spawn", "player_move");

    for event in &events {
        // BUG: `println` is a macro, not a function — needs `!`
        println("{}", event);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vec_macro() {
        let events: Vec<&str> = vec!["player_spawn", "enemy_spawn", "player_move"];
        assert_eq!(events.len(), 3);
        assert_eq!(events[0], "player_spawn");
    }

    #[test]
    fn test_format_macro() {
        let name = "Alice";
        let hp = 100;
        // format! is also a macro — same `!` rule
        let msg = format!("{} has {}hp", name, hp);
        assert_eq!(msg, "Alice has 100hp");
    }
}
