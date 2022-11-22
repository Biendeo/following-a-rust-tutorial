use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    // The move keyword here moves any variables into the scope of the thread. This is because the compiler doesn't know when a thread will end so the ownership needs to move.
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread, here is v {:?}!", i, v);
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::sleep(Duration::from_secs(1));
    println!("hi from the main thread!");

    handle.join().unwrap();

    // tx is the transmitter, rx is the receiver.
    let (tx, rx) = mpsc::channel();
    // You can have multiple producers by just cloning the pairing you get.
    let tx1 = tx.clone();
    
    let handle1 = thread::spawn(move || {
        let val = String::from("sup");
        tx.send(val).unwrap();
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // val's ownership has moved here so you can't use it anymore.
    });

    let handle2 = thread::spawn(move || {
        let vals = vec![
            String::from("hi again"),
            String::from("from again"),
            String::from("the other"),
            String::from("thread again"),
        ];
        for val in vals {
            // This is using the cloned transmitter, but it should work as intended.
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Mutexes are thread safe locks around some data, but the mutex reference itself needs to have an atomic reference counted box.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
            println!("Just moved num to: {}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}