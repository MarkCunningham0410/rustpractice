struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("someoneelse@example.com");

    let user2 = build_user(
        String::from("someonenew@example.com"),
        String::from("somenewusername123"),
    );

    let user3 = User{
        active: user1.active,
        email: String::from("another@example.com"),
        username: user1.username,
        sign_in_count: user1.sign_in_count,
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
