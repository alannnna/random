// EXERCISE 3: Replace three functions with one variadic macro.
// Functions require a fixed number of arguments. To log 1, 2, or 3 fields you'd need
// three separate functions. Macros support repeating patterns with `$( ... ),*`
// meaning "zero or more comma-separated repetitions of this pattern".
// Replace inspect1/inspect2/inspect3 with a single `inspect!` macro.
// Run: cargo test --bin ex03_variadic -p module_11_macros

// BUG: three functions to cover 1, 2, and 3 fields — can't add a 4th without another function
fn inspect1(entity: &str, k1: &str, v1: &str) -> String {
    format!("{}: {}={}", entity, k1, v1)
}

fn inspect2(entity: &str, k1: &str, v1: &str, k2: &str, v2: &str) -> String {
    format!("{}: {}={} {}={}", entity, k1, v1, k2, v2)
}

fn inspect3(entity: &str, k1: &str, v1: &str, k2: &str, v2: &str, k3: &str, v3: &str) -> String {
    format!("{}: {}={} {}={} {}={}", entity, k1, v1, k2, v2, k3, v3)
}

// TODO: write one macro that replaces all three
// macro_rules! inspect {
//     ($entity:expr, $( $key:expr => $val:expr ),* ) => { ... }
// }

fn main() {
    println!("{}", inspect1("Player", "hp", "100"));
    println!("{}", inspect2("Player", "hp", "100", "mp", "50"));
    println!("{}", inspect3("Player", "hp", "100", "mp", "50", "gold", "200"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_field() {
        let s = inspect!("Player", "hp" => "100");
        assert_eq!(s, "Player: hp=100");
    }

    #[test]
    fn test_two_fields() {
        let s = inspect!("Player", "hp" => "100", "mp" => "50");
        assert_eq!(s, "Player: hp=100 mp=50");
    }

    #[test]
    fn test_three_fields() {
        let s = inspect!("Player", "hp" => "100", "mp" => "50", "gold" => "200");
        assert_eq!(s, "Player: hp=100 mp=50 gold=200");
    }

    #[test]
    fn test_four_fields() {
        // A function-based approach would require a fourth function — the macro handles it for free
        let s = inspect!("Enemy", "hp" => "30", "atk" => "5", "def" => "2", "xp" => "10");
        assert_eq!(s, "Enemy: hp=30 atk=5 def=2 xp=10");
    }
}
