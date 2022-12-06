fn main() {
    let mut x = Some(2);
    let y = match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    dbg!(&y);

    if let Some(z) = x {
        dbg!(z);
    }
    if let Option::None = x {
        println!("This should never run!");
    }
    x = Option::None;
    if let Option::None = x {
        println!("This should run!");
    }

    let mut v = vec![1, 3, 7];
    while let Some(a) = v.pop() {
        dbg!(a);
    }

    let v = vec!['b', 'a', 'n', 'a', 'n', 'a'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let (i, j, k) = (1, 2, 3);

    println!("{i}, {j}, {k}");

    match y {
        Some(3) => println!("This line specifically called!"),
        _ => println!("Nope"),
    }

    match i {
        1 | 2 => println!("This matches multiple values!"),
        _ => println!("Nope"),
    }

    match j {
        2..=4 => println!("This matches a range!"),
        _ => println!("Nope"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("{x}, {y}");

    match p {
        Point { x: _, y: 0 } => println!("Nope"),
        Point { x, y: 7 } => println!("Yeah point! x is {x}"),
        _ => println!("Nope"),
    }

    match p {
        Point { x, y: 7 } if x < 5 => println!("x is {x}, low enough!"),
        _ => println!("Nope"),
    }

    match p {
        Point { x: _, y: height_variable @ 3..=7 } => println!("height_variable is {height_variable}"),
        _ => println!("Nope"),
    }

}

struct Point {
    x: i32,
    y: i32,
}