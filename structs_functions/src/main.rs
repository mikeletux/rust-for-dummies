#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.area() >= rect.area()
    }

    fn square(size: u32) -> Self { // N.B: we are not using self as this is a constructor.
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 31,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.can_hold(&rect2) {
        println!("rect2 can be hold in rect1. Area rect2: {}. Area rect1: {}", rect2.area(), rect1.area() )
    }

    let square1 = Rectangle::square(32);
    println!("{square1:#?}")
}