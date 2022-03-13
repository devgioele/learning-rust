enum Color {
    Green = 1,
    Red = 3,
    Blue = -1,
}

enum Box {
    Black(str),
    White(str),
}

fn main() {
    let options = [Some(5), None, Some(-3)];
    for o in options {
        println!("The extracted value of {:?} is {}", o, extract_let_if(o));
    }
}

fn extract(o: Option<i64>) -> i64 {
    match o {
        None => 0,
        Some(v) => v,
    }
}

// Using the let-if notation is shorter and not exhaustive
fn extract_let_if(o: Option<i64>) -> i64 {
    if let Some(v) = o {
        v
    } else {
        0
    }
}
