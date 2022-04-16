mod lib;
use lib::sort::quicksort;

fn main() {
    let mut vec: Vec<f64> = vec![1.0, 9.7, 3.4, 4.0, -3.14];
    quicksort(&mut vec);
    assert_eq!(vec, [-3.14, 1.0, 3.4, 4.0, 9.7]);
}
