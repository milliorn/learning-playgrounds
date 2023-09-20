use std::fs::File;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = 6;
    let z = add(x, y);
    println!("{} + {} = {}", x, y, z);

    let data = "Some data";
    write_to_file(data).expect("Failed to write to file");

    let arr = [1, 2, 3, 4];
    let total = sum(&arr);
    println!("Sum of {:?} is {}", arr, total);

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(5.0);
    println!("Area of circle: {}", area(&circle));
    println!("Area of square: {}", area(&square));

    write_to_file_with_error("example.txt", "Some data").expect("Failed to write to file");

    let arr = [1, 2, 3, 4];
    let key = 3;
    match find_element(&arr, key) {
        Some(item) => println!("Found {} in {:?}", item, arr),
        None => println!("Could not find {} in {:?}", key, arr),
    }

    let income = 1000.0;
    process_income(income).expect("Failed to process income");
    
}

fn add(a: i32, b: i32) -> i32 {
    a + b // Purely based on the input, no side-effects
}

fn write_to_file(data: &str) -> std::io::Result<()> {
    let mut file = File::create("example.txt")?;
    file.write_all(data.as_bytes()) // Has a side-effect (writes to a file)
}

// Calculation: Pure function for summing an array
fn sum(arr: &[i32]) -> i32 {
    arr.iter().fold(0, |acc, &x| acc + x)
}

// You can easily test this
#[test]
fn test_sum() {
    assert_eq!(sum(&[1, 2, 3, 4]), 10);
}

enum Shape {
    Circle(f64),
    Square(f64),
}

// Calculation: Calculate area based on the shape
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14159 * radius * radius,
        Shape::Square(side) => side * side,
    }
}

// Action: Writes a string to a file
fn write_to_file_with_error(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())
}

// Action: Trying to find an element in a list
fn find_element(arr: &[i32], key: i32) -> Option<i32> {
    for &item in arr.iter() {
        if item == key {
            return Some(item);
        }
    }
    None
}

// Calculation
fn calculate_tax(income: f64) -> f64 {
    income * 0.2
}

// Action
fn save_to_database(data: f64) -> Result<(), &'static str> {
    println!("Saving {} to database...", data); // Simulated action
    Ok(())
}

// High-Level Orchestration
fn process_income(income: f64) -> Result<(), &'static str> {
    let tax = calculate_tax(income); // Calculation
    save_to_database(tax) // Action
}
