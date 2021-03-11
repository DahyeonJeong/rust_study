fn main() {
    // instance
    let mut user1 = User {
        email: String::from("sdardew@tistory.com"),
        username: String::from("sdardew")
    };

    println!("{}: {}", user1.username, user1.email);

    user1 = build_user(String::from("sdardew-valley"), String::from("sdardew@mail.com"));

    println!("{}: {}", user1.username, user1.email);

    let user2 = User {
        username: String::from("sdardew"),
        ..user1
    };

}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email
    }
}

// stucture
struct User {
    username: String,
    email: String
}