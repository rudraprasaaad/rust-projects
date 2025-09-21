struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let mut user = User {
        active: true,
        username: String::from("rudraprasaaad"),
        email: String::from("goudarudra23@gmail.com"),
        sign_in_count: 1,
    };

    let another_user = User {
        active: user.active,
        username: String::from("hey"),
        email: String::from("hey@mail.com"),
        sign_in_count: user.sign_in_count,
    };

    println!("{}", another_user.active);
    println!("{}", another_user.username);
    println!("Before updating email {}", another_user.email);
    println!("{}", another_user.sign_in_count);

    println!("{}", user.active);
    println!("{}", user.username);
    println!("Before updating email {}", user.email);
    println!("{}", user.sign_in_count);

    user.email = String::from("rudrag988@gmail.com");
    println!("After updating email {}", user.email);

    let fn_user = build_user(String::from("hey@gmail.com"), String::from("cooldude"));

    println!("{}", fn_user.email);
    println!("{}", fn_user.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
