fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("user123@email.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@email.com");

    let user2 = User {
        email: String::from("hmm@email.com"),
        ..user1
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}