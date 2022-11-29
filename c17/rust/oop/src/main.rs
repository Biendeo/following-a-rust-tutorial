use oop::averaged_collection;

use averaged_collection::AveragedCollection;
use oop::draw::{Screen, SelectBox, Button};
use oop::post::Post;

fn main() {
    let mut x = AveragedCollection::new();
    x.add(5);
    x.add(10);
    x.add(15);
    assert_eq!(10.0, x.average());
    dbg!(&x);

    // This uses dynamic dispatch to allow for `screen.run()` to dynamically call the correct type's `draw()` function. It lacks compile-time optimisations though.
    let screen = Screen {
        components: vec![
            // This SelectBox has private fields so we have to go through the constructor.
            Box::new(SelectBox::new(
                75,
                10,
                vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            )),
            // This Button has public fields so we can construct the struct directly.
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
