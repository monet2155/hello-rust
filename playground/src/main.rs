fn main() {
    let x = String::from("TEST");

    println!("{}", x);

    x.push_str(", TEST2");

    println!("{}", x);
}
