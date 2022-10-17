fn main() {
    let my_struct = MyStruct {};
    let result = my_struct.hello(100);
    println!("Result = {}", result);
}

struct MyStruct {}

trait MyTrait<T> {
    fn hello(&self, item: T) -> T;
}

impl MyTrait<i32> for MyStruct {
    fn hello(&self, item: i32) -> i32 {
        println!("Hello, {}!", item);
        item
    }
}

impl MyTrait<String> for MyStruct {
    fn hello(&self, item: String) -> String {
        todo!()
    }
}
