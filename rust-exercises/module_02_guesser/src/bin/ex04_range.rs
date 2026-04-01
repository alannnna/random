// EXERCISE 4: Fix the range — it's missing the last value.
// `1..5` means 1, 2, 3, 4 (exclusive end).
// `1..=5` means 1, 2, 3, 4, 5 (inclusive end).
// Run: cargo test --bin ex04_range -p module_02_guesser

fn sum_inclusive(start: u32, end: u32) -> u32 {
    let mut total = 0;
    for n in start..end {
        total += n;
    }
    total
}

fn collect_range(start: u32, end: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for n in start..end {
        result.push(n);
    }
    result
}

fn main() {
    println!("Sum 1..=5:  {}", sum_inclusive(1, 5));
    println!("Range 1..=4: {:?}", collect_range(1, 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_inclusive() {
        assert_eq!(sum_inclusive(1, 5), 15);   // 1+2+3+4+5
        assert_eq!(sum_inclusive(1, 10), 55);  // 1..=10
    }

    #[test]
    fn test_collect_range() {
        assert_eq!(collect_range(1, 4), vec![1, 2, 3, 4]);
        assert_eq!(collect_range(5, 5), vec![5]);
    }
}
