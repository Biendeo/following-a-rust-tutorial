pub trait Draw {
    fn draw(&self);
}

// The use of Box<dyn Draw> means that this stores pointers to objects that can draw (allowing for dynamic dispatch).
// This could use a generic type but that would lock the Screen to one singular type.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Button {
    pub fn new(width: u32, height: u32, label: String) -> Button {
        Button {
            width,
            height,
            label
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button with size {}x{} and label {} is drawing!", self.width, self.height, self.label);
    }
}

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl SelectBox {
    pub fn new(width: u32, height: u32, options: Vec<String>) -> SelectBox {
        SelectBox {
            width,
            height,
            options
        }
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox with size {}x{} and options {:?} is drawing!", self.width, self.height, self.options);
    }
}