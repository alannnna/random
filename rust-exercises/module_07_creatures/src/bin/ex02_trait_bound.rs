// EXERCISE 2: Add the missing trait bound.
// Generic functions can accept any type T, but if you call methods on T (like `.name()`),
// you must tell the compiler that T implements the required trait.
// Add the `Describable` trait bound to `print_all`.
// Run: cargo test --bin ex02_trait_bound -p module_07_creatures

use std::fmt;

trait Describable {
    fn name(&self) -> &str;
    fn describe(&self) -> String;
}

struct Goblin { pub hp: u32 }
struct Wizard { pub mp: u32 }

impl Describable for Goblin {
    fn name(&self) -> &str { "Goblin" }
    fn describe(&self) -> String { format!("Goblin with {}hp", self.hp) }
}

impl Describable for Wizard {
    fn name(&self) -> &str { "Wizard" }
    fn describe(&self) -> String { format!("Wizard with {}mp", self.mp) }
}

fn print_all<T>(creatures: &[T]) -> Vec<String> {
    creatures.iter().map(|c| c.describe()).collect()
}

fn loudest_name<T>(creatures: &[T]) -> Option<&str> {
    creatures.iter().map(|c| c.name()).max_by_key(|n| n.len())
}

fn main() {
    let goblins = vec![Goblin { hp: 10 }, Goblin { hp: 20 }];
    for desc in print_all(&goblins) {
        println!("{}", desc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_all() {
        let goblins = vec![Goblin { hp: 10 }, Goblin { hp: 30 }];
        assert_eq!(
            print_all(&goblins),
            vec!["Goblin with 10hp", "Goblin with 30hp"]
        );
    }

    #[test]
    fn test_loudest_name() {
        let creatures: Vec<Box<dyn Describable>> = vec![
            Box::new(Goblin { hp: 10 }),
            Box::new(Wizard { mp: 50 }),
        ];
        assert_eq!(loudest_name(&creatures), Some("Wizard"));
    }
}
