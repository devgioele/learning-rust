fn main() {
    let x = 5;
    let y = 7;
    println!("add_mul({}, {}) = {}", x, y, add_mul(x, y));
}

fn add_mul(x: i64, y: i64) -> i64 {
    (x + y) * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_mul() {
        let x = 3;
        let y = 4;
        let result = 14;
        assert_eq!(
            add_mul(x, y),
            result,
            "testing add_mul with {} and {}, which should be {}",
            x,
            y,
            result
        );
    }
}
