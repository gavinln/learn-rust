fn main() {
    // polymorphism is often synonoymous with inheritance
    // But, it refers to code that can work with multiple types of data
    // Rust using trait objects instead of inheritance to enable polymorphism
    let b = Button {
        width: 3,
        height: 4,
        label: String::from("Ok"),
    };
    let s = SelectBox {
        width: 5,
        height: 2,
        options: vec![String::from("a"), String::from("b")],
    };
    println!("Components {:?} {:?}", b, s);

    let screen = Screen {
        components: vec![Box::new(b), Box::new(s)],
    };
    println!("Screen has {} components", screen.components.len());

    screen.run();

    let b2 = Button {
        width: 3,
        height: 4,
        label: String::from("Ok"),
    };
    let b3 = Button {
        width: 3,
        height: 4,
        label: String::from("Cancel"),
    };
    #[allow(unused_variables)]
    let s2 = SelectBox {
        width: 5,
        height: 2,
        options: vec![String::from("a"), String::from("b")],
    };

    let screen2 = Screen2 {
        components: vec![b2, b3],
        // cannot use different types
        // components: vec![b2, b3],
    };
    println!("Screen has {} components", screen2.components.len());
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw> holds structs that implement the Draw trait
    // trait objects perform dynamic dispath
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("In button draw!")
    }
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("In select_box draw!")
    }
}

// static dispatch as only one type can be stored
pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
