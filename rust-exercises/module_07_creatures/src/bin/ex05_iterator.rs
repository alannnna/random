// EXERCISE 5: The deliverable — implement the Iterator trait.
// To make a type iterable, implement `Iterator` with an `Item` type and a `next()` method.
// `next()` returns `Some(item)` while there are items, then `None` when exhausted.
// Implement Iterator for `AttackSequence` so it produces damage values.
// Run: cargo test --bin ex05_iterator -p module_07_creatures

struct AttackSequence {
    attacks: Vec<u32>,
    position: usize,
}

impl AttackSequence {
    fn new(attacks: Vec<u32>) -> AttackSequence {
        AttackSequence { attacks, position: 0 }
    }
}

// BUG: Iterator is not implemented for AttackSequence
// impl Iterator for AttackSequence {
//     type Item = u32;
//     fn next(&mut self) -> Option<u32> { ... }
// }

fn total_damage(seq: AttackSequence) -> u32 {
    seq.sum()   // BUG: won't compile — sum() requires Iterator
}

fn max_hit(seq: AttackSequence) -> Option<u32> {
    seq.max()   // BUG: same
}

fn main() {
    let seq = AttackSequence::new(vec![10, 25, 5, 30, 15]);
    println!("Total damage: {}", total_damage(seq));

    let seq2 = AttackSequence::new(vec![10, 25, 5, 30, 15]);
    println!("Max hit: {:?}", max_hit(seq2));

    // Once Iterator is implemented, for loops work too:
    let seq3 = AttackSequence::new(vec![5, 10, 15]);
    for dmg in seq3 {
        println!("Hit for {}", dmg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_damage() {
        let seq = AttackSequence::new(vec![10, 25, 5, 30, 15]);
        assert_eq!(total_damage(seq), 85);
    }

    #[test]
    fn test_max_hit() {
        let seq = AttackSequence::new(vec![10, 25, 5, 30, 15]);
        assert_eq!(max_hit(seq), Some(30));
    }

    #[test]
    fn test_empty() {
        let seq = AttackSequence::new(vec![]);
        assert_eq!(total_damage(seq), 0);
        assert_eq!(max_hit(AttackSequence::new(vec![])), None);
    }

    #[test]
    fn test_for_loop_collects() {
        let seq = AttackSequence::new(vec![1, 2, 3]);
        let collected: Vec<u32> = seq.collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }
}
