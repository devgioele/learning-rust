fn main() {
    println!("Hello, world!");

    another_function();

    println!("{}", add_greater_3(2));

    testing_control_flow();

    loop_through();

    let result = return_from_loop();
    println!("Result = () ? {}", result == ())
}

fn another_function() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

// Function defining its return type
fn add_greater_3(x: i32) -> i32 {
    if x > 3 {
        x + 1
    }
    else {
         x
    }
}

fn testing_control_flow() {
    println!("Testing control flow...");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn loop_through() {
    for n in 2..7 {
        println!("n = {}", n);
    }
}

fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}