fn main() {
    let x = "Banana!"; // The type is inferred to be an i32.
    println!("The value of x is: {x}");
    print_type_of(&x);
    // x = 6; // This is illegal as x is not mutable.

    let mut x = 4; // Shadows the last usage of x.
    println!("The value of x is: {x}");
    print_type_of(&x);
    x = 7;
    println!("The value of x is: {x}");
    // x = 7.5; // Can't assign another type.

    const A_CONSTANT : i32 = 2 * 3 * 5; // Type cannot be inferred.
    println!("The value of A_CONSTANT is: {A_CONSTANT}");
    // A_CONSTANT = 12; // Can't change constants either.

    {
        let x = 10.5; // Shadowing lets you change type.
        println!("The value of x is: {x}");
        print_type_of(&x);
    }
    println!("The value of x is: {x}"); // This maintains the above value of 7 because shadowing still creates new variables.

    let x: isize = 0x12345678;
    println!("The value of x is: {x}");
    print_type_of(&x); // Interestingly maintains the name "isize".

    // The following bit runs with "cargo run --release" but not "cargo run", overflows are caught in debug mode.
    // let mut x: i32 = 0x12345678;
    // x = x * 10;
    // println!("The value of x is: {x}");

    let c = '🦎';
    println!("The value of c is: {c}");

    let t = (1, 5.0, "Hi");
    // println!("The value of t is: {t}"); // Compound types cannot be printed directly.
    print_type_of(&t);

    let (a, b, c) = t;
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");

    // You can access tuple members like this (can't use '.' in the format string but this does the job).
    println!("The value of t.0 is: {}", t.0);
    println!("The value of t.1 is: {}", t.1);
    println!("The value of t.2 is: {}", t.2);

    let a = [5, 3, 1, 2, 4];
    // println!("The value of a is: {a}"); // Can't print arrays directly.
    print_type_of(&a);

    println!("The value of a[0] is: {}", a[0]);
    // println!("The value of a[5] is: {}", a[5]); // Array out of bounds at runtime.

    let y = {
        let x = 3;
        x + 1
    }; // This is effectively an inline expression.
    println!("The value of y is: {y}");
    print_type_of(&y);

    println!("The value of a_cool_function is: {}", a_cool_function());

    // There is no multiline comment.

    if y == 4 {
        println!("y == 4");
    } else {
        println!("This shouldn't run!");
    }

    let b: bool = true;
    if b {
        println!("Bools work!");
    }

    let mut counter = 0;
    let r = loop {
        counter += 1;
        println!("The value of counter is: {counter}");

        if counter >= 10 {
            break counter * 17;
        }
    };
    println!("The value of r is: {r}");

    while counter < 15 {
        counter += 1;
        println!("The value of counter is: {counter}");
    }

    // For is exclusively for extracting from an iterator
    for element in a {
        println!("The value of element is: {element}");
    }

    // let 🚗 = '🐫'; // They specifically have a compiler message: "identifiers cannot contain emoji"
    // println!("The value of 🚗 is: {🚗}");
}


// This is a really neato function that uses a generic to know the type of a variable ahead of time.
fn print_type_of<T>(_: &T) {
    println!("Its type is: {}", std::any::type_name::<T>())
}

// Functions must declare return types with the arrow, the last statement is implicitly the return.
fn a_cool_function() -> i32 {
    500; // You can just have statements in space like this.
    600 // Specifically no semicolon here, otherwise it doesn't return it.
}