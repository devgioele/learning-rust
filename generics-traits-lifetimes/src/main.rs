use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

struct Shape;

impl Shape {
    fn new() -> Self {
        Shape
    }
}

fn main() {
    let p1 = Pair::new(9, 4);
    p1.cmp_display();
    let _p2 = Pair::new(Shape::new(), Shape::new());
    // The following does not work: p2 does not implement the function `cmp_display`
    // p2.cmp_display();
}
