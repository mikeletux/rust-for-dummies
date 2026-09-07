struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = build_user(String::from("mikeletux"), String::from("miguelsamamerino@gmail.com"));

    user1.email = String::from("hehe@hehe.es");

    let user2 = User {
        email: String::from("msama@cube.xyz"),
        ..user1 // the remaining fields should have the same values as fields in the given instance.
    };

    // This code will not work because when creating user2, username from user1 was moved into user2
    // println!("{}", user1.username);
    //println!("{}", user2.username);

    // This code will work since email value was not moved from user1 to user2
    println!("{}", user1.email);
    println!("{}", user2.email);

    // active and sign_in_count will be still valid since they are scalar values and they implements the Copy trait.

}

// If the fields of the struct and the parameters are the same, we can use the Field Init Shorthand
// so we do not write username: username and so on all the time.
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}