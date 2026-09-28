struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual; // Unit-like structs - Used when implementing traits (don't have any data that you want to store in the type itself)

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Destructure
    let Point(x, y, z) = origin;
}
