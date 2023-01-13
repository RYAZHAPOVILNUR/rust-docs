fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        active: bool
    }

    let user1 = User {
        email: String::from("werwr@sdf.fsd"),
        username: String::from("Test user"),
        active: true
    };

    // user1.active = false;

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true
        }
    }

    let user2 = build_user(String::from("test@email.com"), String::from("John"));

    let user3 = User {
        email: String::from("hello"),
        ..user1
    };

    println!("{:?}", user3);

    struct Product (String, String, i32);

    let product1 = Product (
        String::from("qwe"),
        String::from("sdf"),
        50
    );

    println!("{}, {}, {}", product1.0, product1.1, product1.2);
}


