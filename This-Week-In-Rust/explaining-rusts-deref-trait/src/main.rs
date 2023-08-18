use std::ops::Deref;

struct Curse<T>(T);

impl<T> Deref for Curse<T> {
    type Target = usize;

    fn deref(&self) -> &<Self as Deref>::Target {
        &42 // wat
    }
}

fn main() {
    println!("Hello, world!");

    // Dereferencing means accessing the data that’s referred to by a reference. Let’s work through an example, so that it’s easy to see what I mean.
    // Let’s say we have a variable a, that is holding a value 123:

    let a = 123;
    // We can now take a reference, also called borrowing in Rust terminology, to a and store it as the variable b. The ampersand (&) symbol is used to do that:
    let b = &a;
    // Now that we have a reference, we need to dereference it to get the value 123 again. To dereference a reference, add the asterisk (* ) symbol to the left of it.
    let c = *b;

    assert_eq!(a, c);

    // In the example below, the Curse<T> is a tuple struct that ignores the original value. During the dereference step, it returns a reference to 42, irrespective  of whatever was provided. I think of operator overloading as a sharp knife feature. You’re allowed to do whatever you want, but you risk cutting yourself.
    let d = 123;
    let e = Curse(d);
    let f = *e;

    println!("{a} {c}");
    assert_eq!(f, 42);
    println!("All assertions passed.");
}
