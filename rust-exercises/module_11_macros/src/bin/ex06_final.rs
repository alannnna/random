// EXERCISE 6: The deliverable — a game debug inspector.
// Fix the three marked bugs to complete the inspector:
//   1. log_event! must capture the call site (not a helper function's location)
//   2. inspect! must accept any number of key=>value fields
//   3. check_eq! must show expression text on failure, not just values
// Run: cargo test --bin ex06_final -p module_11_macros
//
// When all three macros work, `run_game` produces structured output like:
//   [ex06_final.rs:80] player_spawn
//   Player: hp=100 mp=50 gold=0
//   OK

// BUG 1: log_event is a function — always reports this file at this line, not the call site
fn log_event(event: &str) -> String {
    format!("[{}:{}] {}", file!(), line!(), event)
}
// macro_rules! log_event {
//     ($event:expr) => { ... }
// }

// BUG 2: inspect only handles exactly two fields — crashes or won't compile for any other count
fn inspect(entity: &str, k1: &str, v1: &str, k2: &str, v2: &str) -> String {
    format!("{}: {}={} {}={}", entity, k1, v1, k2, v2)
}
// macro_rules! inspect {
//     ($entity:expr, $( $key:expr => $val:expr ),* ) => { ... }
// }

// BUG 3: check_equal loses expression text — failure just prints raw numbers
fn check_equal(left: i32, right: i32) -> Result<(), String> {
    if left != right {
        Err(format!("assertion failed: {} != {}", left, right))
    } else {
        Ok(())
    }
}
// macro_rules! check_eq {
//     ($left:expr, $right:expr) => { ... }
// }

// TODO: define the `define_event!` macro here, then it will be available to the invocations below
// macro_rules! define_event {
//     ($name:ident) => { ... }
// }

define_event!(SpawnEvent);
define_event!(DamageEvent);
define_event!(DeathEvent);

fn run_game() -> Vec<String> {
    let mut log = Vec::new();

    log.push(log_event!("player_spawn"));  // BUG 1: fix log_event so this uses call-site location

    let player_hp = 100;
    let player_mp = 50;
    let player_gold = 0;
    log.push(inspect!("Player",             // BUG 2: fix inspect! so this compiles with 3 fields
        "hp"   => &player_hp.to_string(),
        "mp"   => &player_mp.to_string(),
        "gold" => &player_gold.to_string()
    ));

    let max_hp = 100;
    match check_eq!(player_hp, max_hp) {    // BUG 3: fix check_eq! so failure shows expression names
        Ok(())  => log.push("OK".to_string()),
        Err(e)  => log.push(e),
    }

    let event = SpawnEvent::new("Player", 1000);
    log.push(event.summary());

    log
}

fn main() {
    for line in run_game() {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_event_call_site() {
        let msg = log_event!("player_spawn");
        assert!(msg.contains("ex06_final.rs"), "expected filename in: {}", msg);
        assert!(msg.contains("player_spawn"), "expected event in: {}", msg);
    }

    #[test]
    fn test_inspect_three_fields() {
        let s = inspect!("Player", "hp" => "100", "mp" => "50", "gold" => "0");
        assert_eq!(s, "Player: hp=100 mp=50 gold=0");
    }

    #[test]
    fn test_inspect_one_field() {
        let s = inspect!("Enemy", "hp" => "30");
        assert_eq!(s, "Enemy: hp=30");
    }

    #[test]
    fn test_check_eq_pass() {
        assert!(check_eq!(100, 100).is_ok());
    }

    #[test]
    fn test_check_eq_fail_shows_expressions() {
        let player_hp = 95;
        let max_hp = 100;
        let err = check_eq!(player_hp, max_hp).unwrap_err();
        assert!(err.contains("player_hp"), "missing left expr in: {}", err);
        assert!(err.contains("max_hp"), "missing right expr in: {}", err);
    }

    #[test]
    fn test_define_event() {
        let e = SpawnEvent::new("Player", 1000);
        assert_eq!(e.summary(), "SpawnEvent(entity=Player, t=1000)");
        let d = DamageEvent::new("Goblin", 1050);
        assert_eq!(d.summary(), "DamageEvent(entity=Goblin, t=1050)");
    }

    #[test]
    fn test_run_game() {
        let log = run_game();
        assert_eq!(log.len(), 4);
        assert!(log[0].contains("player_spawn"));
        assert_eq!(log[1], "Player: hp=100 mp=50 gold=0");
        assert_eq!(log[2], "OK");
        assert_eq!(log[3], "SpawnEvent(entity=Player, t=1000)");
    }
}
