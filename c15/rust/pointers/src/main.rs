#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    // This lets you use the * operator on types.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use std::rc::Rc;

#[derive(Debug)]
enum ReferenceList {
    RCons(i32, Rc<ReferenceList>),
    RNil,
}

use crate::ReferenceList::{RCons, RNil};


fn main() {
    let b = Box::new(5); // This use case is very arbitrary but it shows how boxes can be made.
    println!("b = {b}"); // Boxed variables automatically unbox when printing.

    // This is a functional style list where it is defined as an element, and then the rest of the list (or Nil, the end).
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); // Syntax could be easier.
    dbg!(list);

    let b = MyBox::new(100);
    let x = 100;
    println!("Does x == *b? {}", x == *b);

    let _c = CustomSmartPointer {
        data: String::from("hi there!"),
    }; 
    // This will eventually drop and call their appropriate methods.
    let _d = CustomSmartPointer {
        data: String::from("hello!"),
    };
    println!("CustomSmartPointers created");
    drop(_d); // This explicitly drops one. _d.drop() exists but works a little differently and the compiler tells you off for trying to use that.

    // Because of ownership, you'll need reference counted pointers to do stuff like this. These have the advantages over references because they don't specify lifetime.
    let a = Rc::new(RCons(5, Rc::new(RCons(10, Rc::new(RNil)))));
    let b = RCons(3, Rc::clone(&a));
    let c = RCons(4, Rc::clone(&a));

    dbg!(a);
    dbg!(b);
    dbg!(c);
}
