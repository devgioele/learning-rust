mod lib;

use quicksort::sort::{quicksort, quicksort_concurrent};
use std::sync::Mutex;
use threadpool::ThreadPool;

fn main() {
    let mut vec1: Vec<f64> = vec![1.0, 9.7, 3.4, 4.0, -3.14];
    let mut vec2 = vec1.clone();
    let vec_sorted = [-3.14, 1.0, 3.4, 4.0, 9.7];
    let pool = ThreadPool::new(2);

    // Serial
    quicksort(&mut vec1);
    assert_eq!(vec1, vec_sorted);

    // Concurrent
    quicksort_concurrent(&mut pool, &mut vec2);
    assert_eq!(vec2, vec_sorted);
}
