// EXERCISE 2: Fix the double mutable borrow.
// You can only have ONE mutable reference to a value at a time.
// Here, `first` and `last` are both mutable references into the same Vec simultaneously.
// Fix it by using `v.swap(0, last_idx)` instead of two simultaneous mutable refs.
// Run: cargo test --bin ex02_double_mut -p module_04_borrowing

fn swap_first_last(v: &mut Vec<i32>) {
    let last_idx = v.len() - 1;
    let first = &mut v[0];           // mutable borrow of v
    let last  = &mut v[last_idx];
    std::mem::swap(first, last);
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    swap_first_last(&mut v);
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let mut v = vec![1, 2, 3, 4, 5];
        swap_first_last(&mut v);
        assert_eq!(v, vec![5, 2, 3, 4, 1]);
    }

    #[test]
    fn test_swap_two_elements() {
        let mut v = vec![10, 20];
        swap_first_last(&mut v);
        assert_eq!(v, vec![20, 10]);
    }
}
