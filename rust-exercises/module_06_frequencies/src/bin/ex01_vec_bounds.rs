// EXERCISE 1: Fix the out-of-bounds panic.
// `v[i]` panics if `i >= v.len()`. For safe access, use `v.get(i)` which returns
// an `Option<&T>` — `Some(&value)` if in bounds, `None` if not.
// Fix `nth` to use .get() and handle the None case.
// Run: cargo test --bin ex01_vec_bounds -p module_06_frequencies

fn nth(v: &[i32], i: usize) -> i32 {
    v[i]   // BUG: panics if i is out of bounds — use .get() instead
}

fn top_two(v: &[i32]) -> (Option<i32>, Option<i32>) {
    let mut sorted = v.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    // BUG: both of these panic if sorted has fewer than 2 elements
    (Some(sorted[0]), Some(sorted[1]))
}

fn main() {
    let scores = vec![42, 17, 99, 55, 8];
    println!("Item 2: {:?}", nth(&scores, 2));
    println!("Item 9: {:?}", nth(&scores, 9));  // should return 0 or default, not panic

    let (first, second) = top_two(&scores);
    println!("Top two: {:?}, {:?}", first, second);

    let one = vec![100];
    let (f, s) = top_two(&one);
    println!("Top two of [100]: {:?}, {:?}", f, s);  // should not panic
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_in_bounds() {
        assert_eq!(nth(&[10, 20, 30], 1), 20);
    }

    #[test]
    fn test_nth_out_of_bounds() {
        assert_eq!(nth(&[10, 20, 30], 99), 0);  // 0 as a safe default
    }

    #[test]
    fn test_top_two_enough() {
        assert_eq!(top_two(&[3, 1, 4, 1, 5]), (Some(5), Some(4)));
    }

    #[test]
    fn test_top_two_one_element() {
        assert_eq!(top_two(&[42]), (Some(42), None));
    }

    #[test]
    fn test_top_two_empty() {
        assert_eq!(top_two(&[]), (None, None));
    }
}
