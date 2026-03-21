// EXERCISE 1: Implement the Display trait.
// `println!("{}", value)` requires the type to implement `std::fmt::Display`.
// Without it, you'll get a compile error: "doesn't implement Display".
// Implement Display for `Creature` so it prints as "Goblin (hp=15)".
// Run: cargo test --bin ex01_display -p module_07_creatures

use std::fmt;

struct Creature {
    name: String,
    hp: u32,
}

// BUG: Display is not implemented for Creature
// impl fmt::Display for Creature {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         ...
//     }
// }

fn main() {
    let c = Creature { name: "Goblin".to_string(), hp: 15 };
    println!("{}", c);  // BUG: won't compile — Creature doesn't implement Display
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let goblin = Creature { name: "Goblin".to_string(), hp: 15 };
        let dragon = Creature { name: "Dragon".to_string(), hp: 200 };
        assert_eq!(format!("{}", goblin), "Goblin (hp=15)");
        assert_eq!(format!("{}", dragon), "Dragon (hp=200)");
    }
}
