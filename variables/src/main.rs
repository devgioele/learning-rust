fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let tup1 = (400, -3, 2);
    let (_x, _y, _z) = tup1;
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
}