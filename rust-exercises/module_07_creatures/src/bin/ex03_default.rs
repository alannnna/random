// EXERCISE 3: Derive Default.
// The `Default` trait provides a zero-value constructor: `T::default()`.
// You can derive it automatically if all fields also implement Default.
// Fix the code by deriving Default for the structs that need it.
// Run: cargo test --bin ex03_default -p module_07_creatures

#[derive(Debug)]
struct Stats {
    hp: u32,
    mp: u32,
    attack: u32,
}

#[derive(Debug)]
struct Creature {
    name: String,
    stats: Stats,
    alive: bool,
}

fn spawn(name: &str) -> Creature {
    Creature {
        name: name.to_string(),
        ..Creature::default()
    }
}

fn main() {
    let goblin = spawn("Goblin");
    println!("{:?}", goblin);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_stats() {
        let s = Stats::default();
        assert_eq!(s.hp, 0);
        assert_eq!(s.mp, 0);
        assert_eq!(s.attack, 0);
    }

    #[test]
    fn test_spawn() {
        let c = spawn("Troll");
        assert_eq!(c.name, "Troll");
        assert_eq!(c.stats.hp, 0);
        assert!(!c.alive);
    }
}
