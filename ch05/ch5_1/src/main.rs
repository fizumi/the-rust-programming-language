fn main() {
    println!("Hello, world!");
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn immutable_user() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // user1.email = String::from("anotheremail@example.com"); // error
}

fn mutable_user() {
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Using the Field Init Shorthand when Variables and Fields Have the Same Name
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


// Creating Instances From Other Instances With Struct Update Syntax
fn creating_instances() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // ✔ struct update syntax
    };
}


// Using Tuple Structs without Named Fields to Create Different Types
fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // let white: Color = Point(255, 255, 255); // ❌ 同じ形でも違う型なら error
}