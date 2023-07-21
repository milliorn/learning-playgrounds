use std::num::Wrapping;

// Let’s define a function that simply adds two numbers and then take a look at the generated assembly:

pub fn addition(x: u8, y: u8) -> u8 {
    x + y
}

fn main() {
    println!("Hello, world!");

    // Consider a minimal Rust application which causes integer overflow:
    // let x: u8 = 255 + 1;
    // println!("{}", x);

    // This is an example of one of the many static checks the Rust compiler performs on any code it compiles. Taking a closer look at the error message, we are informed that this check is enabled due to #[deny(arithmetic_overflow)] being on by default. This would imply we can disable this check by adding an allow annotation to the offending line. Let’s try that:

    #[allow(arithmetic_overflow)]
    let x: u8 = 255 + 1;
    println!("{}", x);

    // These functions are equivalent to the wrapping_ functions except they also return a Boolean value which indicates whether or not overflow occurred. This may be particularly useful when implementing emulators for example as many CPUs have a flag that must be set whenever an instruction causes overflow.

    let (result, overflowed) = (250_u8).overflowing_add(10); // 4, true
    println!(
        "sum is {} where overflow {} occur",
        result,
        if overflowed { "did" } else { "did not" }
    );

    // Perhaps instead of wrapping values, we want to handle overflow as a special case. This can be done using the checked_ functions as follows:

    match (100_u8).checked_add(200) {
        Some(result) => println!("{result}"),
        None => panic!("overflowed!"),
    }

    // If there are numerous places in a given code base where overflow may occur then the above method will quickly become quite verbose and perhaps difficult to work with. It also entails the risk of missing an instance where overflow could occur, and thus risking a runtime panic (depending on the build profile). Fortunately, Rust offers a solution in the form of the type Wrapping<T>. This type allow the normal arithmetic operators (+, /, etc.) to be used while ensuring values are automatically wrapped around whenever integer overflow occurs. Let’s try it out:
    let mut x = Wrapping(125_u8);

    x + Wrapping(200); // 69
    x - Wrapping(200); // 181
    x *= 5; // if we mutate the variable x then we can use primitive integer types - x is now 113

    // x / 5; // error! careful - we can only use primitives when we're assigning (i.e., using +=, -=, etc.)
}
