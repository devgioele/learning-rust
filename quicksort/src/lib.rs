pub mod sort {
    use rayon::join;
    pub use rayon::prelude::*;

    fn swap(arr: &mut [f64], a: usize, b: usize) {
        let old_a = arr[a];
        arr[a] = arr[b];
        arr[b] = old_a;
    }

    /// Unstable partitioning algorithm.
    ///
    /// Return the index of the pivot, such that the left partition is <= the
    /// pivot and the right partition is > the pivot.
    fn partition_hoare(arr: &mut [f64], low: usize, high: usize) -> usize {
        let pivot = arr[(high + low) / 2];
        // Set indices taking possible overflows into account
        let (mut left, mut skip_left) = if low == usize::MIN {
            (low, true)
        } else {
            (low - 1, false)
        };
        let (mut right, mut skip_right) = if high == usize::MAX {
            (high, true)
        } else {
            (high + 1, false)
        };
        // Scan from outside inwards
        loop {
            loop {
                // Do not increment `left` if not necessary at the first iteration
                if skip_left {
                    skip_left = false;
                } else {
                    left += 1;
                }
                if arr[left] >= pivot {
                    break;
                }
            }
            loop {
                // Do not decrement `right` if not necessary at the first iteration
                if skip_right {
                    skip_right = false;
                } else {
                    right -= 1;
                }
                if arr[right] <= pivot {
                    break;
                }
            }
            // If indices crossed
            if left >= right {
                break;
            }
            swap(arr, left, right);
        }
        right
    }

    fn quicksort_seq_rec(arr: &mut [f64], low: usize, high: usize) {
        // Base case
        if low >= high {
            return;
        }
        // Continue by induction
        let pivot = partition_hoare(arr, low, high);
        quicksort_seq_rec(arr, low, pivot);
        quicksort_seq_rec(arr, pivot + 1, high);
    }

    /// Unstable sorting of the given array.
    pub fn quicksort_seq(arr: &mut [f64]) {
        quicksort_seq_rec(arr, 0, arr.len() - 1);
    }

    /// Sorts the given array using potential parallelism.
    /// This means that at least 1 thread is used and further threads might
    /// be used if they are available and idle.
    ///
    /// # Examples
    ///
    /// ```
    /// # use quicksort::sort::quicksort;
    /// let mut arr = [3.0, 3.0, 9.0, 3.0, 7.0];
    /// quicksort(&mut arr);
    /// assert_eq!(arr, [3.0, 3.0, 3.0, 7.0, 9.0]);
    /// ```
    pub fn quicksort(arr: &mut [f64]) {
        let low = 0;
        let high = arr.len() - 1;
        // Base case
        if low >= high {
            return;
        }
        // Partition
        let pivot = partition_hoare(arr, low, high);
        // Split the array without copying it (uses unsafe code under the hood)
        let (left, right) = arr.split_at_mut(pivot + 1);
        // Continue by induction
        join(|| quicksort(left), || quicksort(right));
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod partition {
            use super::*;

            fn partition_hoare_whole(arr: &mut [f64]) -> usize {
                partition_hoare(arr, 0, arr.len() - 1)
            }

            #[test]
            fn hoare_one() {
                let mut arr = [-3.3];
                let pivot = partition_hoare_whole(&mut arr);
                assert_eq!(pivot, 0);
                assert_eq!(arr, [-3.3]);
            }

            #[test]
            fn hoare_two_unsorted() {
                let mut arr = [9.0, 8.0];
                let pivot = partition_hoare_whole(&mut arr);
                assert_eq!(pivot, 0);
                assert_eq!(arr, [8.0, 9.0]);
            }

            #[test]
            fn hoare_even_sorted() {
                let mut arr = [1.0, 2.4, 3.0, 7.0];
                let pivot = partition_hoare_whole(&mut arr);
                assert_eq!(pivot, 1);
                assert_eq!(arr, [1.0, 2.4, 3.0, 7.0]);
            }

            #[test]
            fn hoare_even_sorted_slice() {
                let mut arr = [1.0, 2.4, 3.0, 7.0, 16.4, 902.1, -703.2, 9.2];
                let pivot = partition_hoare(&mut arr, 1, 4);
                assert_eq!(pivot, 2);
                assert_eq!(arr, [1.0, 2.4, 3.0, 7.0, 16.4, 902.1, -703.2, 9.2]);
            }

            #[test]
            fn hoare_equal() {
                let mut arr = [3.0, 3.0, 3.0];
                let pivot = partition_hoare_whole(&mut arr);
                assert_eq!(pivot, 1);
                assert_eq!(arr, [3.0, 3.0, 3.0]);
            }

            #[test]
            fn hoare_odd_sorted() {
                let mut arr = [2.3, 3.0, 4.0];
                let pivot = partition_hoare_whole(&mut arr);
                assert_eq!(pivot, 1);
                assert_eq!(arr, [2.3, 3.0, 4.0]);
            }

            #[test]
            fn hoare_even_unsorted() {
                let mut arr = [1.0, 7.1, 2.2, 8.0];
                let pivot = partition_hoare_whole(&mut arr);
                assert_eq!(pivot, 1);
                assert_eq!(arr, [1.0, 2.2, 7.1, 8.0]);
            }

            #[test]
            fn hoare_odd_unsorted() {
                let mut arr = [9.2, 3.1, 4.0];
                let pivot = partition_hoare_whole(&mut arr);
                assert_eq!(pivot, 0);
                assert_eq!(arr, [3.1, 9.2, 4.0]);
            }
        }

        #[test]
        fn quicksort_seq_one() {
            let mut arr = [9.7];
            quicksort_seq(&mut arr);
            assert_eq!(arr, [9.7]);
        }

        #[test]
        fn quicksort_seq_two_unsorted() {
            let mut arr = [3.4, 1.0];
            quicksort_seq(&mut arr);
            assert_eq!(arr, [1.0, 3.4]);
        }

        #[test]
        fn quicksort_seq_even_sorted() {
            let mut arr = [1.0, 9.7, 3.4, 4.0];
            quicksort_seq(&mut arr);
            assert_eq!(arr, [1.0, 3.4, 4.0, 9.7]);
        }

        #[test]
        fn quicksort_seq_odd_sorted() {
            let mut arr = [1.0, 9.7, 3.4, 4.0, -3.14];
            quicksort_seq(&mut arr);
            assert_eq!(arr, [-3.14, 1.0, 3.4, 4.0, 9.7]);
        }

        #[test]
        fn quicksort_serial() {
            let mut arr1 = [1.0, 9.7, 3.4, 4.0];
            let mut arr2 = arr1.clone();
            assert_eq!(quicksort_seq(&mut arr1), quicksort(&mut arr2));
        }

        #[test]
        fn quicksort_even_unsorted_concurrent() {
            let mut arr = [1.0, 9.7, 3.4, 4.0];
            quicksort(&mut arr);
            assert_eq!(arr, [1.0, 3.4, 4.0, 9.7]);
        }

        #[test]
        fn quicksort_odd_unsorted_concurrent() {
            let mut arr = [1.0, 9.7, 3.4, 4.0, -3.14];
            quicksort(&mut arr);
            assert_eq!(arr, [-3.14, 1.0, 3.4, 4.0, 9.7]);
        }
    }
}
