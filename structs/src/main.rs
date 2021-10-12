struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername1234"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@examplecom");
}
