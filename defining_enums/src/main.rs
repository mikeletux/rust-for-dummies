
// The name of each enum variant that we define also becomes a function that constructs an instance of the enum. 
// That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("yuhuuuu!!!")
    }
}

fn main() {
    let home = IpAddrKind::V4(192, 168, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Holly molly"));
    m.call();

    let some_number = Some(5);
    let some_chat = Some('e');

    let absent_number: Option<i32> = None;
}
