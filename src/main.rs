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
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
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
}
