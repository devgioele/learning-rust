use quicksort::sort::median_rec;
use quicksort::threads::Pool;

fn main() {
    let values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 9.0];

    // TODO:
    // 1. Start the sorting
    // 2. Wait for the sorting to complete

    let pool = Pool::new(5);

    pool.execute(Box::new(move || {
        println!("median of degree 1 = {}", median_rec(&values, 1));
        println!("median of degree 2 = {}", median_rec(&values, 2));
    }));

    loop {}
}
