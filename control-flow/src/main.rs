fn main() {
    let mut user1 = build_user(String::from("Elpis"), String::from("elpis@gmail.com"));

    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
