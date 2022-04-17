extern crate scoped_threadpool;

pub mod sort {
    use scoped_threadpool::Pool;

    /// Computes the media for a given arrtor of numbers.
    /// The media is the value separating the higher half from
    /// the lower half of a data sample.
    ///
    /// # Examples
    ///
    /// A data set with an odd number of values:
    /// ```
    /// use quicksort::sort::median;
    /// let arr = [1.0,3.0,3.0,6.0,7.0,8.0,9.0];
    /// let median = median(&arr);
    /// assert_eq!(median, 6.0);
    /// ```
    ///
    /// A data set with an even number of values:
    /// ```
    /// use quicksort::sort::median;
    /// let arr = [1.0,2.0,3.0,4.0,5.0,6.0,8.0,9.0];
    /// let median = median(&arr);
    /// assert_eq!(median, 4.5);
    /// ```
    pub fn median(arr: &[f64]) -> f64 {
        let len = arr.len();
        return if len % 2 == 0 {
            (arr[len / 2 - 1] + arr[len / 2]) / 2f64
        } else {
            arr[(len + 1) / 2 - 1]
        };
    }

    fn swap(arr: &mut [f64], a: usize, b: usize) {
        let old_a = arr[a];
        arr[a] = arr[b];
        arr[b] = old_a;
    }

    /// Unstable partitioning algorithm.
    ///
    /// returns: index of the pivot, such that the left partition is <= the pivot and
    /// the right partition is > the pivot.
    fn partition_hoare(arr: &mut [f64], low: usize, high: usize) -> usize {
        let pivot = arr[(high + low) / 2];
        // Set indices taking possible overflows into account
        let (mut left, mut skip_left) = if low == 0 {
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

    fn quicksort_rec(arr: &mut [f64], low: usize, high: usize) {
        // Base case
        if low >= high {
            return;
        }
        // Continue by induction
        let pivot = partition_hoare(arr, low, high);
        quicksort_rec(arr, low, pivot);
        quicksort_rec(arr, pivot + 1, high);
    }

    /// Unstable sorting of the given array.
    pub fn quicksort(arr: &mut [f64]) {
        quicksort_rec(arr, 0, arr.len() - 1);
    }

    fn quicksort_concurrent_rec(pool: &mut Pool, arr: &mut [f64]) {
        let low = 0;
        let high = arr.len() - 1;
        // Partition
        let pivot = partition_hoare(arr, low, high);
        // Split the array without copying it (uses unsafe code under the hood)
        let (left, right) = arr.split_at_mut(pivot + 1);
        // Continue by induction
        pool.scoped(|scope| {
            if low < pivot {
                scope.execute(move || {
                    quicksort_concurrent_rec(pool, left);
                });
            }
            if pivot + 1 < high {
                scope.execute(move || {
                    quicksort_concurrent_rec(pool, right);
                });
            }
        });
    }

    pub fn quicksort_concurrent(pool: &mut Pool, arr: &mut [f64]) {
        if arr.len() > 1 {
            quicksort_concurrent_rec(pool, arr);
        }
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
        fn quicksort_one() {
            let mut arr = [9.7];
            quicksort(&mut arr);
            assert_eq!(arr, [9.7]);
        }

        #[test]
        fn quicksort_two_unsorted() {
            let mut arr = [3.4, 1.0];
            quicksort(&mut arr);
            assert_eq!(arr, [1.0, 3.4]);
        }

        #[test]
        fn quicksort_even_sorted() {
            let mut arr = [1.0, 9.7, 3.4, 4.0];
            quicksort(&mut arr);
            assert_eq!(arr, [1.0, 3.4, 4.0, 9.7]);
        }

        #[test]
        fn quicksort_odd_sorted() {
            let mut arr = [1.0, 9.7, 3.4, 4.0, -3.14];
            quicksort(&mut arr);
            assert_eq!(arr, [-3.14, 1.0, 3.4, 4.0, 9.7]);
        }

        #[test]
        fn quicksort_concurrent_serial() {
            let mut arr1 = [1.0, 9.7, 3.4, 4.0];
            let mut arr2 = arr1.clone();
            let pool = Mutex::new(ThreadPool::new(2));
            assert_eq!(quicksort(&mut arr1), quicksort_concurrent(&pool, &mut arr2));
        }

        #[test]
        fn quicksort_even_unsorted_concurrent() {
            let mut arr = [1.0, 9.7, 3.4, 4.0];
            let pool = Mutex::new(ThreadPool::new(2));
            quicksort_concurrent(&pool, &mut arr);
            assert_eq!(arr, [1.0, 3.4, 4.0, 9.7]);
        }

        #[test]
        fn quicksort_odd_unsorted_concurrent() {
            let mut arr = [1.0, 9.7, 3.4, 4.0, -3.14];
            let pool = Mutex::new(ThreadPool::new(2));
            quicksort_concurrent(&pool, &mut arr);
            assert_eq!(arr, [-3.14, 1.0, 3.4, 4.0, 9.7]);
        }
    }
}
