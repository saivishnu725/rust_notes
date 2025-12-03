// Result enum is used for error handling
fn divide(a: u8, b: u8) -> Result<u8, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("---------------------");

    // variables and mutability
    let x: u8 = 5;
    let mut y: u8 = 10;
    println!("immutable x = {}, mutable y = {}", x, y);
    // x += 1; // this will give error as x is immutable
    y += 1;
    println!("After incrementing: x = {}(can not incr) y = {}", x, y);

    println!("---------------------");

    // arrays
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    let two = &arr[2..4]; // 4 is not inclusive. to include 4, use 2..=4
    println!("Slice of array: {:?}", two);

    println!("---------------------");

    // tuples
    let tup: (u8, f32, char, bool) = (1, 2.4, 'a', true);
    println!("Tuple: {:?}", tup);
    println!(
        "Accessing tuple elements: first = {}, second = {}, third = {}, fourth = {}",
        tup.0, tup.1, tup.2, tup.3
    );
    let (a, b, c, d) = tup; // destructuring
    println!(
        "Destructured tuple: a = {}, b = {}, c = {}, d = {}",
        a, b, c, d
    );

    println!("---------------------");

    // strings
    let mut s: String = String::from("hello");
    println!("String: {}", s);
    s.push_str(", world!");
    println!("String after push_str(): {}", s);

    // string slices &str
    let sa: &str = &s[0..5]; // 5 is not inclusive. to include 5, use 0..=5
    println!("Slice of string: {}", sa);

    println!("---------------------");

    // loops
    // for
    for i in 0..5 {
        // 5 is not inclusive. to include 5, use 0..=5
        println!("i in for loop = {}", i);
    }
    // while
    let mut count: u8 = 0;
    while count < 5 {
        println!("count in while loop = {}", count);
        count += 1;
    }
    // loop
    let mut n: u8 = 0;
    loop {
        if n >= 5 {
            break;
        }
        println!("n in loop = {}", n);
        n += 1;
    }

    println!("---------------------");

    // error handling
    // let num: Result<u8, &str> = Ok(10); // change to Err to see error
    let num: Result<u8, &str> = Err("An error occurred"); // uncomment to see error case
    match num {
        Ok(n) => println!("The number is: {}", n),
        Err(e) => println!("Error: {}", e), // handle the error without crashing
    }

    println!("---------------------");

    // unrecoverable error (panic!)
    // change the condition to true to see panic
    let condition = false; // change to true to see panic
    // let condition = true; // uncomment to see panic
    if condition {
        panic!("This is a panic! The program will crash.");
    } else {
        println!("No panic occurred. Program continues.");
    }

    println!("---------------------");

    // Demonstrate divide function with error handling
    let mut a: u8 = 50;
    let mut b: u8 = 5;
    match divide(a, b) {
        Ok(result) => println!("{a} / {b} = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    a = 5;
    b = 0;
    match divide(a, b) {
        Ok(result) => println!("{a} / {b} = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
