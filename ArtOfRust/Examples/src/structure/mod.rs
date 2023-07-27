pub mod tuple;
pub mod rectangle;
pub mod unit;
pub mod method;

pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn user() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}

pub fn mutable_user() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

pub fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}


pub fn build_user_2(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

pub fn main() {
    let mut user = build_user("paradox@email.com".to_string(), "paradox".to_string());
}