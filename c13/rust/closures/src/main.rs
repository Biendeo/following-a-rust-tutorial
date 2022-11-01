use std::thread;
use std::time::Duration;

fn main() {
    let sleep_closure = || {
        println!("This is a closure!");
        thread::sleep(Duration::from_secs(1));
        println!("And the closure is still going!");
    };

    sleep_closure();

    let closure_syntax_1 = |x: i32| -> i32 { x + 1 };
    let closure_syntax_2 = |x: i32| x + 1;

    println!("Both functions have the same result: {} and {}", closure_syntax_1(5), closure_syntax_2(5));

    let mut x = 5;

    // This has to be mutable because it performs mutable operations internally.
    let mut capturing_closure = || x = x + 5;

    capturing_closure();

    println!("Closures can borrow their outer scope: {x}");

    // How do I pass mutable variables into closures?
    // let another_closure = |&y| y = y + 5;

    // another_closure(&x);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // println!("Before calling closure: {:?}", list); // This line is an immutable borrow but `list` has technically been borrowed by the closure.
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    println!("The list total is {total}");

    let v1_iter = v1.iter();

    let mapping: Vec<i32> = v1_iter.map(|x| x * 50).collect();
    println!("The mapped list is now {:?}", mapping);

    let v1_iter = v1.iter();

    let filtering: Vec<&i32> = v1_iter.filter(|x| **x <= 2).collect();
    println!("The filtered list is now {:?}", filtering);
}
