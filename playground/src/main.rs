#[derive(Debug)]
struct User {
    username: String,
    email: String,
}

fn main() {
    let x = String::from("TEST");

    println!("{}", x);

    // x.push_str(", TEST2");

    // println!("{}", x);

    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let user1 = User {
        username: String::from("username1"),
        email: String::from("email1"),
    };

    let user2 = User {
        username: String::from("user212"),
        ..user1
    };

    println!("{:?}", user2);
}
