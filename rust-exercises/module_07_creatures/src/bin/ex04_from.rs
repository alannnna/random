// EXERCISE 4: Implement From for type conversion.
// `From<T>` lets you define how to convert one type into another.
// Once you implement `From<A> for B`, you get `B::from(a)` and `a.into()` for free.
// Implement `From<&str> for CreatureKind` and `From<CreatureKind> for String`.
// Run: cargo test --bin ex04_from -p module_07_creatures

#[derive(Debug, PartialEq)]
enum CreatureKind {
    Goblin,
    Dragon,
    Wizard,
    Unknown(String),
}

// BUG: From<&str> for CreatureKind is not implemented
// impl From<&str> for CreatureKind { ... }

// BUG: From<CreatureKind> for String is not implemented
// impl From<CreatureKind> for String { ... }

fn classify(name: &str) -> CreatureKind {
    CreatureKind::from(name)   // BUG: won't compile without the From impl
}

fn kind_name(kind: CreatureKind) -> String {
    String::from(kind)         // BUG: won't compile without the From impl
}

fn main() {
    let kind = classify("dragon");
    println!("Kind: {:?}", kind);
    println!("Name: {}", kind_name(kind));
    println!("Unknown: {:?}", classify("vampire"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(CreatureKind::from("goblin"), CreatureKind::Goblin);
        assert_eq!(CreatureKind::from("dragon"), CreatureKind::Dragon);
        assert_eq!(CreatureKind::from("wizard"), CreatureKind::Wizard);
        assert_eq!(CreatureKind::from("vampire"), CreatureKind::Unknown("vampire".to_string()));
    }

    #[test]
    fn test_into_string() {
        assert_eq!(String::from(CreatureKind::Goblin), "goblin");
        assert_eq!(String::from(CreatureKind::Dragon), "dragon");
        assert_eq!(String::from(CreatureKind::Unknown("bat".to_string())), "bat");
    }

    #[test]
    fn test_classify_and_name() {
        assert_eq!(kind_name(classify("wizard")), "wizard");
    }
}
