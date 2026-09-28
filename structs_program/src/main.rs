#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(30 * scale), // prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.
        height: 50,
    };

    println!("rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}