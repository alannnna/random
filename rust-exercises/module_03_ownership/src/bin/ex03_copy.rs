// EXERCISE 3: Understand Copy types.
// Some types (i32, f64, bool, char, etc.) implement the Copy trait.
// Copy types are duplicated automatically on assignment/pass — no move occurs.
// Structs are NOT Copy by default. Add #[derive(Copy, Clone)] to make Point copyable.
// Run: cargo test --bin ex03_copy -p module_03_ownership

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn distance_from_origin(p: Point) -> f64 {
    ((p.x * p.x + p.y * p.y) as f64).sqrt()
}

fn describe(p: Point) -> String {
    format!("({}, {})", p.x, p.y)
}

fn analyze(p: Point) -> (String, f64) {
    let desc = describe(p);             // BUG: p is moved here
    let dist = distance_from_origin(p); // BUG: p was already moved
    (desc, dist)
}

fn main() {
    let p = Point { x: 3, y: 4 };
    let (desc, dist) = analyze(p);
    println!("{} is {} units from the origin", desc, dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze() {
        let p = Point { x: 3, y: 4 };
        let (desc, dist) = analyze(p);
        assert_eq!(desc, "(3, 4)");
        assert_eq!(dist, 5.0);
    }

    #[test]
    fn test_copy_behavior() {
        let p = Point { x: 0, y: 0 };
        let _d = distance_from_origin(p);
        // After fixing, p should still be usable here because Point is Copy
        let _desc = describe(p);
    }
}
