fn main() {
    // Some is effectively Rust's optional type.
    let mut x = Some(5);
    println!("{x:?}");

    // This is the empty type (note the same type so I can reassign it). It's scoped to Option::None.
    x = None;
    println!("{x:?}");

    let ip_kind = IpAddrKind::V4;
    println!("{ip_kind:?}");

    let home = IpAddr::V4(127, 0, 0, 1);
    println!("{home:?}");
    
    // you can decompose enums like this.
    // Also below is the double default "_" to catch all remaining values, and "()" to do nothing with them.
    match home {
        IpAddr::V4(a, b, c, d) => println!("{a}.{b}.{c}.{d}"),
        _ => (),
    }
    
    // You can also use this syntax for single matches.
    if let IpAddr::V4(a, b, c, d) = home {
        println!("{a}.{b}.{c}.{d}");
    }
    
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{loopback:?}");

    // And obviously this won't execute.
    if let IpAddr::V4(a, b, c, d) = loopback {
        println!("{a}.{b}.{c}.{d}");
    }

    println!("IPV4 version is {}", IpAddrKind::V4.version());
    println!("IPV4 version is {}", IpAddrKind::V6.version());

}

// Enums can be defined as such. They encode some unimportant underlying value.
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

impl IpAddrKind {
    fn version(&self) -> i32 {
        match self {
            IpAddrKind::V4 => 4,
            IpAddrKind::V6 => 6,
        }
    }
}

// Enums can also hold any arbitrary types internally
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}