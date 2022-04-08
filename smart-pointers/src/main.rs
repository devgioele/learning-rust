use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

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

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    //*** Using a custom smart pointer
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

    //*** Using the Reference Counter type
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let _c = Cons(3, Rc::clone(&a));
    // Overwriting the second element of list `b`
    if let Cons(_, mut l2) = b {
        l2 = Rc::new(Cons(-7, Rc::new(Nil)));
        // Reading the first item of the newly inserted list
        if let Cons(item, _) = *l2 {
            println!("First item of newly inserted list: {}", item);
        }
    }
}

// fn main() {
//     let x = 5;
//     let y = &x;
//
//     assert_eq!(5, x);
//     // Why does this work?
//     assert_eq!(&5, y);
// }
