use crate::utils::is_sorted;
use rand::{seq::SliceRandom, thread_rng};

/// Sorts the provided slice with Tutor Sort in-place.
///
/// Time complexity in the worst case is `O(infinity)`.
pub fn tutor_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut rng = thread_rng();
    while !is_sorted(arr) {
        arr.shuffle(&mut rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tutor_sort_works() {
        let mut arr = [2, 3, 1];
        tutor_sort(&mut arr);
        assert!(is_sorted(&arr))
    }
}
