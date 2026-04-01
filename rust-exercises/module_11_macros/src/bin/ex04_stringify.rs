// EXERCISE 4: Use stringify! to capture expression text.
// Functions receive values — they have no idea what expression produced them.
// `stringify!(expr)` is a built-in macro that returns the source text of `expr` as a &str,
// without evaluating it. This is how `assert_eq!` can print "left: hp + bonus, right: max_hp"
// instead of just "left: 105, right: 100".
// Replace `check_equal` with a `check_eq!` macro that shows the expression text on failure.
// Run: cargo test --bin ex04_stringify -p module_11_macros

fn check_equal(left: i32, right: i32) -> Result<(), String> {
    if left != right {
        // Can't print expression names — only raw numbers
        Err(format!("assertion failed: {} != {}", left, right))
    } else {
        Ok(())
    }
}

// TODO: replace check_equal with a macro using stringify!
// macro_rules! check_eq {
//     ($left:expr, $right:expr) => { ... }
// }

fn main() {
    let hp = 95;
    let bonus = 10;
    let max_hp = 100;

    // With a function, failure prints: "assertion failed: 105 != 100"
    // With the macro, failure should print: "assertion failed: hp + bonus != max_hp (105 != 100)"
    match check_equal(hp + bonus, max_hp) {
        Ok(()) => println!("OK"),
        Err(e) => println!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_passes_when_equal() {
        let hp = 100;
        let max_hp = 100;
        assert!(check_eq!(hp, max_hp).is_ok());
    }

    #[test]
    fn test_fails_with_expression_text() {
        let hp = 95;
        let bonus = 10;
        let max_hp = 100;
        let err = check_eq!(hp + bonus, max_hp).unwrap_err();
        // Must include the source text of both expressions
        assert!(err.contains("hp + bonus"), "missing left expr in: {}", err);
        assert!(err.contains("max_hp"), "missing right expr in: {}", err);
        // And the actual values
        assert!(err.contains("105"), "missing left value in: {}", err);
        assert!(err.contains("100"), "missing right value in: {}", err);
    }

    #[test]
    fn test_fails_with_literal_text() {
        let err = check_eq!(1 + 1, 3).unwrap_err();
        assert!(err.contains("1 + 1"), "missing expr in: {}", err);
        assert!(err.contains('3'), "missing right value in: {}", err);
    }
}
