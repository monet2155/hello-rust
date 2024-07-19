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
}
