use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Constants aren’t just immutable by default—they’re always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    let x = x + 1;
    println!("The value of x is {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "  ";
    let spaces = spaces.len();

    println!("The length of spaces is: {spaces}");

    // https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is {x}");
    println!("The value of y is {y}");
    println!("The value of y is {z}");

    let z: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = z.0;

    let six_point_four = z.1;

    let one = z.2;

    println!("The value of five_hundred is {five_hundred}");
    println!("The value of six_point_four is {six_point_four}");
    println!("The value of one is {one}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    println!("Value of first is {first}");
    println!("Value of second is {second}");

    println!("Enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
