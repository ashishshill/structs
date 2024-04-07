// struct also share refarance with index number
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // we create here instance
    let mut user1 = User {
        email: String::from("boggyman@gmail.com"),
        username: String::from("boggyman123"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("johnwich123");

    let user2 = build_user(String::from("john@gmail.com"), String::from("john123"));

    let user3 = User {
        email: String::from("bale@gmail.com"),
        username: String::from("bale123"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
