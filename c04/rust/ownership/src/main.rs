fn main() {
    {
        let _s = "hello"; // Underscore variables don't get warned if they're unused.
    }
    // Can't refer to _s anymore here.

    let s1 = String::from("hello");
    let s2 = "hello";
    print_type_of(&s1); // This is type alloc::string::String.
    print_type_of(&s2); // This is type &str.

    let mut s = String::from("hello"); // A String can be mutated.
    s.push_str(", world!");
    println!("{s}");

    let i = 5;
    steal_the_int(i);
    println!("No I still have: {i}");

    steal_the_string(s);
    // println!("I don't have {s} anymore!"); // ERROR: borrow of moved value

    s = give_a_string();
    println!("I did manage to get: {s}");

    refer_to_the_string(&s);
    println!("I still have: {s}!");

    // Only the latest reference to a mutable variable can be used.
    let s2 = &mut s;
    println!("{s2}"); // This is fine.
    // s.push_str(" again"); // This gets angry because s2 is borrowing it.

    // let s3 = &s; // This doesn't let you borrow immutably while it's borrowed mutably.

    // let i2 = &mut i; // Obviously can't mutable reference immutable variables.

    let mut s = String::from("hi there");
    {
        let s2 = &s;
        let s3 = &s;
        println!("I can use {s2} and {s3} fine.");

        // s.push_str(" bro"); // Even though s2 and s3 are immutable, you can't be mutable on the original while these are in scope.
    }

    // But once the references are out of scope, you can act mutable again.
    s.push_str(" bro");
    println!("{s}");

    let slice = &s[3..8];
    println!("Slice is: {slice}");
    print_type_of(&slice); // Note this is the exact same type as the string literals.

    // s.push_str("ski"); // This is illegal though because slice is a reference to "some" of s, and therefore we can't modify.

    let p = string_without_first_four_letters(&s);
    println!("The string without the first four letters is: {p}");


    let p = string_without_first_four_letters(&slice);
    println!("The next string without the first four letters is: {p}");


}

fn print_type_of<T>(_: &T) {
    println!("Its type is: {}", std::any::type_name::<T>())
}

// This copies the int to the function because ints have the Copy trait.
fn steal_the_int(i: i32) {
    println!("I tried stealing {i} but did I really?");
}

// This moves the string's ownership to this function, which disposes it when the function ends.
fn steal_the_string(s: String) {
    println!("I have now stolen the string: {s}");
}

// This constructs a string and passes it out.
fn give_a_string() -> String {
    String::from("here you go")
}

// This refers to the string but doesn't remove the original reference.
fn refer_to_the_string(s: &String) {
    println!("I have now \"borrowed\" the string: {s}");
}

// This tries to return a reference to a variable that leaves scope, which is a big nono.
// fn dangling_reference() -> &String {
//     let s = String::from("Hi");
//     &s
// }

// This returns a substring (slice). The larger "String" types are also "str" types, so this works for both.
fn string_without_first_four_letters(s: &str) -> &str {
    &s[4..]
}