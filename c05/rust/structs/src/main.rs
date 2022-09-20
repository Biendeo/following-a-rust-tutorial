fn main() {
    let user1 = User {
        // Rust errors if you don't initialise everything.
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{user1:?}"); // Debug formatter is invoked with the ':?'.

    // To modify member fields, you need to have a mutable variable.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("fsinatra@email.com"), String::from("chairmanoftheboard"));
    println!("{user2:?}");

    let user3 = User {
        email: String::from("epresley@email.com"),
        ..user2
    };
    println!("{user3:?}");
    dbg!(&user3); // This is a different method for debugging. It prints to stderr.

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::square(40);
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// Structs can be declared later in a file after they're used.
#[derive(Debug)] // This allows the debug printer to handle printing this struct.
#[allow(dead_code)] // This disables the warning of not using the fields for anything.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // Since the variable is the same name as the field, this shorthand is available.
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Rust does not have classes, but it lets you attach methods to structs with the impl blocks.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // This defines methods separately to the structure.
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle { // You can have multiple impls.
    fn square(size: u32) -> Self { // Methods that don't have self as a parameter are static.
        Self {
            width: size,
            height: size,
        }
    }
}