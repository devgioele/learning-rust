use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Return A REFERENCE TO
        // the first element of the tuple struct,
        // which is the only one
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // What Rust runs instead of `*y` is `*(y.deref())`.
    // The dereference operator `*` after calling `deref` is
    // necessary, because `deref` returns a reference to the
    // value.
    assert_eq!(5, *y);
    // Why does this work?
    assert_eq!(&5, y.deref());
}

// fn main() {
//     let x = 5;
//     let y = &x;
//
//     assert_eq!(5, x);
//     // Why does this work?
//     assert_eq!(&5, y);
// }
