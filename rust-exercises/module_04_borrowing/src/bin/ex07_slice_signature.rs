// EXERCISE 7: Prefer &[T] over &Vec<T>.
// `&Vec<i32>` only accepts references to Vecs.
// `&[i32]` accepts Vec references, array references, and any slice — much more flexible.
// A &Vec<i32> automatically coerces to &[i32], but not vice versa.
// Fix the functions to take &[i32] instead of &Vec<i32>.
// Run: cargo test --bin ex07_slice_signature -p module_04_borrowing

fn sum_first_three(data: &Vec<i32>) -> i32 {
    data[0] + data[1] + data[2]
}

fn contains(data: &Vec<i32>, target: i32) -> bool {
    data.iter().any(|&x| x == target)
}

fn main() {
    let v = vec![10, 20, 30, 40, 50];
    let arr = [1, 2, 3, 4, 5];

    println!("Sum first 3 of vec:   {}", sum_first_three(&v));
    println!("Sum first 3 of array: {}", sum_first_three(&arr));

    println!("Vec contains 20:   {}", contains(&v, 20));
    println!("Array contains 3:  {}", contains(&arr, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_vec() {
        let v = vec![10, 20, 30, 40, 50];
        assert_eq!(sum_first_three(&v), 60);
    }

    #[test]
    fn test_sum_array() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_first_three(&arr), 6);  // works after fix
    }

    #[test]
    fn test_contains() {
        let arr = [10, 20, 30];
        assert!(contains(&arr, 20));
        assert!(!contains(&arr, 99));
    }
}
