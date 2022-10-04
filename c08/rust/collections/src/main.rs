fn main() {
    let mut v: Vec<i32> = Vec::new(); // The has to be here.

    let v2 = vec![1, 2, 3]; // This macro also verifies the values match types.
    println!("v2 is {:?}", v2); // The default formatter doesn't handle vectors.

    v.push(5);
    v.push(6);
    v.push(7);

    {
        let mut x = &v[2];
        println!("&v[2] is {x}");

        x = &1;
        println!("&v[2] is now {x}");
    }
    println!("v is now {:?}", v); // This does not change the underlying list.

    let result = v.get(3);
    match result {
        Some(_) => println!("x exists!"),
        None => println!("x does not exist!"),
    }

    for i in &mut v {
        *i += 5;
    }
    println!("v is now {:?}", v);

    let s = String::from("🍕");
    println!("s is {s}");

    let s2 = s + " is my favourite food."; // s is moved now, watch out!
    println!("s2 is {s2}");

    let s3 = format!("{s2} What is your favourite?"); // s2 is copied and still works.
    println!("s3 is {s3}");
    
    // Strings cannot easily be indexable. You can't just get the 0th character.
    // You also can't slice in between a multi-byte character like 🍕.
    for b in s2.bytes() { // This properly splits multi byte characters.
        println!("b is {b}");
    }

    for c in s2.chars() { // This properly handles multi byte characters.
        println!("c is {c}");
    }

    use std::collections::HashMap; // This isn't imported by default.
    let mut scores = HashMap::new();
    print_type_of(&scores);
    scores.insert(String::from("Blue"), 10); // Interestingly, commenting out this line gives a compiler error because the compiler can't figure out the type of scores when it's needed.
    scores.insert(String::from("Red"), 5);
    // scores.insert(String::from("Yellow"), "🟨"); // The type is known now so it prevents non-integer values at compile time.

    for (key, value) in &scores { // This ordering changes when you re-run the program.
        println!("key is {key} and value is {value}");
    }

    scores.insert(String::from("Blue"), 25); // Overwriting is easy.
    scores.entry(String::from("Blue")).or_insert(50); // And inserting if not present.
    scores.entry(String::from("Yellow")).or_insert(50);

    println!("scores is {scores:?}");
}

fn print_type_of<T>(_: &T) {
    println!("Its type is: {}", std::any::type_name::<T>());
}