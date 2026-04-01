// EXERCISE 3: Fix the broken iterator chain.
// Iterators are lazy — they do nothing until consumed. Chains of .map()/.filter()
// must end with a consumer like .collect(), .sum(), .count(), etc.
// Fix the two broken chains below.
// Run: cargo test --bin ex03_iterator_chain -p module_06_frequencies

fn only_evens(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .map(|&x| x)
        .filter(|x| x % 2 == 1)
        .collect()
}

fn doubled_words_longer_than(words: &[&str], min_len: usize) -> Vec<String> {
    words
        .iter()
        .filter(|w| w.len() > min_len)
        .map(|w| w.to_uppercase())
        .collect()
}

fn main() {
    println!("{:?}", only_evens(&[1, 2, 3, 4, 5, 6]));
    println!("{:?}", doubled_words_longer_than(&["hi", "hello", "hey", "howdy"], 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only_evens() {
        assert_eq!(only_evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(only_evens(&[1, 3, 5]), vec![] as Vec<i32>);
    }

    #[test]
    fn test_doubled_words() {
        let result = doubled_words_longer_than(&["hi", "hello", "hey", "howdy"], 3);
        assert_eq!(result, vec!["HELLO HELLO", "HOWDY HOWDY"]);
    }
}
