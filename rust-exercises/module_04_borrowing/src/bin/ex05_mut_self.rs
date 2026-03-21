// EXERCISE 5: Fix the method receiver.
// `&self` gives read-only access to the struct's fields.
// `&mut self` gives mutable access, needed when modifying fields.
// Fix `increment` and `reset` — they modify `self.count` but don't declare `&mut self`.
// Run: cargo test --bin ex05_mut_self -p module_04_borrowing

struct Counter {
    count: u32,
    label: String,
}

impl Counter {
    fn new(label: &str) -> Counter {
        Counter { count: 0, label: label.to_string() }
    }

    fn increment(&self) {        // BUG: modifies self.count but only has &self
        self.count += 1;
    }

    fn reset(&self) {            // BUG: same
        self.count = 0;
    }

    fn value(&self) -> u32 {
        self.count
    }

    fn label(&self) -> &str {
        &self.label
    }
}

fn main() {
    let counter = Counter::new("clicks");
    counter.increment();
    counter.increment();
    counter.increment();
    println!("{}: {}", counter.label(), counter.value());
    counter.reset();
    println!("After reset: {}", counter.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut counter = Counter::new("hits");
        counter.increment();
        counter.increment();
        counter.increment();
        assert_eq!(counter.value(), 3);
        assert_eq!(counter.label(), "hits");
        counter.reset();
        assert_eq!(counter.value(), 0);
    }
}
