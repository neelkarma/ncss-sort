use rand::{seq::SliceRandom, thread_rng};

fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    if arr.is_empty() {
        return true;
    };

    let mut prev = &arr[0];

    for item in arr.iter().skip(1) {
        if prev > item {
            return false;
        }

        prev = item;
    }

    true
}

/// Sorts the provided slice with Tutor Sort in-place.
///
/// Time complexity in the worst case is `O(infinity)`.
pub fn tutor_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut rng = thread_rng();
    while !is_sorted(arr) {
        arr.shuffle(&mut rng);
    }
}

/// Sorts the provided `Vec` with Curran Sort in-place.
///
/// Time complexity in the worst case is `O(n)`.
pub fn curran_sort<T: PartialOrd>(arr: &mut Vec<T>) {
    while !is_sorted(arr) {
        arr.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tutor_sort_works() {
        let mut arr = [2, 3, 1];
        tutor_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn curran_sort_works() {
        let mut arr = vec![2, 3, 1, 4, 5];
        curran_sort(&mut arr);
        assert_eq!(arr, vec![2, 3]);
    }
}
