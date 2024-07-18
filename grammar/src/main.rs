fn main() {
    // variable
    let x = 5;
    println!("The value of x is : {x}");
    // this line has compile error!
    // x is immutable
    // x = 5;
    // println!("The value of x is : {x}");

    let mut mutable_x = 5;
    println!("The value of mutable_x is : {mutable_x}");
    mutable_x = 7;
    println!("The value of mutable_x is : {mutable_x}");

    // constant
    // constant can't used with mut, and requires type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    // Using shadowing can change type of variable with the same name
    let spaces = "   ";
    let spaces = spaces.len();
    // But mutable variable cannot changed type

    // ***** scalar variable *****
    // integer
    // integer overflow make panic. but release build has not panic, occur overflow (In u8, 256 -> 0 257 -> 1)

    // floating-point
    let x = 2.0; // f64

    // bool
    let f: bool = true; // 1byte

    // charactor
    let c: char = 'z';
    let thumbs_up = '👍'; // it can be charactor!

    // ***** compound type *****
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring
    let (x, y, z) = tup;
    println!("The value of y is : {y}");
    // or using index
    println!("The value of y is : {0}", tup.1);

    // If tuple has not any value, it called unit
    let unit_tuple: () = ();
    // If expression not returns any value, returns unit implicitly

    // array
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    // or use default value
    let c = [3; 5]; // c has five threes

    // and access like this
    let first = a[0];

    // If access out of index(like using user input), rust make panic

    // ***** function *****
    another_function();
    has_parameter_function(15);

    // ***** expression and statement *****
    // statement doesn't return value
    // let x = (let y = 6) == NOT WORKING!

    // expression
    let exp_x = {
        let x = 3;
        x + 1 // not using semicolon
    };

    println!("The value of exp_x is : {exp_x}"); // exp_x is 4!!!

    // function who return value
    let return_function = five();

    // ***** flow control expression *****
    let flow_number = 3;

    if flow_number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if flow_number < 5 {
        // ...
    } else if flow_number < 3 {
        // ...
    } else {
        // ...
    }

    let tri_like_if = if flow_number > 5 { 1 } else { 2 }; // do not use diffrent types in conditions

    // ***** loop *****
    let mut counter = 0;

    let loop_result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // loop result send with break {expression}
        }
    };

    println!("The loop result is {loop_result}");

    // labeled loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up; // THIS IS AWESOME!!
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // while
    let mut while_number = 3;
    while while_number != 0 {
        println!("while number : {while_number}");
        while_number -= 1;
    }
    println!("LIFTOFF!!!");

    // for (with collection)
    let for_array = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is : {element}");
    }

    for number in (1..4).rev() {
        // [1,2,3].reverse()
        println!("for loop with temp array : {number}");
    }
}

// function
fn another_function() {
    println!("Another function.");
}

fn has_parameter_function(x: i32) {
    println!("Parameter value is {x}");
}

fn five() -> i32 {
    5 // what..

    // If you want to return value, write value to expression
}
