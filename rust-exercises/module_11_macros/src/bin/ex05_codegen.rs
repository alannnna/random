// EXERCISE 5: Generate repetitive structs with a macro.
// When many types share the same shape, you'd normally copy-paste the struct and impl.
// A macro can generate all of it from a compact declaration, eliminating the duplication.
// Replace the three hand-written event structs with a single `define_event!` macro invocation.
// Run: cargo test --bin ex05_codegen -p module_11_macros

#[derive(Debug, PartialEq)]
struct SpawnEvent {
    entity: String,
    timestamp: u64,
}

impl SpawnEvent {
    fn new(entity: &str, timestamp: u64) -> Self {
        SpawnEvent { entity: entity.to_string(), timestamp }
    }
    fn summary(&self) -> String {
        format!("SpawnEvent(entity={}, t={})", self.entity, self.timestamp)
    }
}

#[derive(Debug, PartialEq)]
struct DamageEvent {
    entity: String,
    timestamp: u64,
}

impl DamageEvent {
    fn new(entity: &str, timestamp: u64) -> Self {
        DamageEvent { entity: entity.to_string(), timestamp }
    }
    fn summary(&self) -> String {
        format!("DamageEvent(entity={}, t={})", self.entity, self.timestamp)
    }
}

#[derive(Debug, PartialEq)]
struct DeathEvent {
    entity: String,
    timestamp: u64,
}

impl DeathEvent {
    fn new(entity: &str, timestamp: u64) -> Self {
        DeathEvent { entity: entity.to_string(), timestamp }
    }
    fn summary(&self) -> String {
        format!("DeathEvent(entity={}, t={})", self.entity, self.timestamp)
    }
}

// TODO: delete the three structs above and replace with this macro + three invocations:
// macro_rules! define_event {
//     ($name:ident) => { ... }
// }
// define_event!(SpawnEvent);
// define_event!(DamageEvent);
// define_event!(DeathEvent);

fn main() {
    let e = SpawnEvent::new("Player", 1000);
    println!("{}", e.summary());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_event() {
        let e = SpawnEvent::new("Player", 1000);
        assert_eq!(e.summary(), "SpawnEvent(entity=Player, t=1000)");
    }

    #[test]
    fn test_damage_event() {
        let e = DamageEvent::new("Goblin", 1050);
        assert_eq!(e.summary(), "DamageEvent(entity=Goblin, t=1050)");
    }

    #[test]
    fn test_death_event() {
        let e = DeathEvent::new("Goblin", 1100);
        assert_eq!(e.summary(), "DeathEvent(entity=Goblin, t=1100)");
    }

    #[test]
    fn test_equality() {
        let a = SpawnEvent::new("Player", 1000);
        let b = SpawnEvent::new("Player", 1000);
        assert_eq!(a, b);
    }
}
