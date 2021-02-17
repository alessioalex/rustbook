struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // field init shorthand below (with email and username fields)
    User {
        // email: email,
        // username: username,
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    /*
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // below -> struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    */

    let user1 = build_user(String::from("m@example.com"), String::from("admin"));
}
