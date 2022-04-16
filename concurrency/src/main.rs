use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel::<String>();
    let tx2 = tx1.clone();

    // Producer
    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];

        for val in vals {
            tx1.send(val.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Producer
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Consumer
    for received in rx {
        println!("Got: {}", received);
    }
}
