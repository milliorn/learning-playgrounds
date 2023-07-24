// Define a function called "baseline" that takes a reference to a string (slice of characters)
// and returns an integer (i64) as the result.
fn baseline(input: &str) -> i64 {
    // Initialize a mutable variable "res" to store the result, and set it to 0.
    let mut res = 0;

    // Iterate over the bytes of the input string.
    for b in input.bytes() {
        // Match the current byte with different cases:
        match b {
            // If the byte is equal to 's' (the ASCII value of 's'), increment the "res" variable by 1.
            b's' => res += 1,

            // If the byte is equal to 'p' (the ASCII value of 'p'), decrement the "res" variable by 1.
            b'p' => res -= 1,

            // For any other byte, continue to the next iteration of the loop.
            _ => continue,
        }
    }

    // Return the final value of "res" as the result of the function.
    res
}

// Define a function called "opt1" that takes a reference to a string (slice of characters)
fn opt1_idiomatic(input: &str) -> i64 {
    input // Return the input string
        .bytes() // Convert the string to a sequence of bytes
        .map(|b| match b {
            // For each byte, match it with different cases:
            b's' => 1,  // If the byte is equal to 's' (the ASCII value of 's'), return 1.
            b'p' => -1, // If the byte is equal to 'p' (the ASCII value of 'p'), return -1.
            _ => 0,     // For any other byte, return 0.
        }) // Return the sequence of integers
        .sum() // Sum up all the integers in the sequence
}

// Define a function called "opt2" that takes a reference to a string (slice of characters)
fn opt2_count_s(input: &str) -> i64 {
    let n_s = input.bytes().filter(|&b| b == b's').count(); // Count the number of 's' in the input string
    (2 * n_s) as i64 - input.len() as i64 // Return the result
}

// Define a function called "opt3" that takes a reference to a string (slice of characters)
fn opt3_count_s_branchless(input: &str) -> i64 {
    let n_s = input.bytes().map(|b| (b & 1) as i64).sum::<i64>(); // Count the number of 's' in the input string
    (2 * n_s) - input.len() as i64 // Return the result
}

// The main entry point of the program.
fn main() {
    // Print "Hello, world!" to the console.
    println!("Hello, world!");

    // Call the "baseline" function with the argument "hello world" and store the result in variable "a".
    let a = baseline("hello world");

    // Print the value of variable "a" to the console.
    println!("a = {}", a);

    // Call the "baseline" function with the argument "hello world" and store the result in variable "b".
    let b = baseline("spp");

    // Print the value of variable "b" to the console.
    println!("b = {}", b);

    // Call the "opt1" function with the argument "hello world" and store the result in variable "c".
    let c = opt1_idiomatic("hell porld");

    // Print the value of variable "c" to the console.
    println!("c = {}", c);

    // Call the "opt2" function with the argument "hello world" and store the result in variable "d".
    let d = opt2_count_s("hello world");

    // Print the value of variable "d" to the console.
    println!("d = {}", d);

    // Call the "opt3" function with the argument "hello world" and store the result in variable "e".
    let e = opt3_count_s_branchless("hello world");

    // Print the value of variable "e" to the console.
    println!("e = {}", e);
}
