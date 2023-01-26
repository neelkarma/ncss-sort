use crate::utils::is_sorted;

/// Sorts the provided `Vec` with Curran Sort in-place.
///
/// Time complexity in the worst case is `O(n)`.
pub fn curran_sort<T: PartialOrd>(arr: &mut Vec<T>) {
    while !is_sorted(arr) {
        arr.pop();
    }
}

mod tests {
    use super::*;

    #[test]
    fn curran_sort_works() {
        let mut arr = vec![2, 3, 1, 4, 5];
        curran_sort(&mut arr);
        assert!(is_sorted(&arr))
    }
}
