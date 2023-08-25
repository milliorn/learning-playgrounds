fn main() {
    println!("Hello, world!");
    println!("{}", add(1, 2));

    // Assign closure to the add_closure variable
    let add_closure = |a: i32, b: i32| -> i32 { a + b };
    let mut sixty_six = add_closure(42, 24);
    println!("{}", sixty_six);

    //Types for a and b and return value omitted
    let add_closure = |a, b| {
        a + b //return type inferred from this expression
    };
    //Types for a and b inferred as i32 from the call site
    let b = add_closure(42, 42);
    println!("{}", b);

    //Braces omitted
    let add_closure = |a, b| a + b;
    sixty_six = add_closure(42, 24);
    println!("{}", sixty_six);

    let i = 42;
    let capture_i = || println!("{i}");
    capture_i(); //prints 42

    let add_5 = create_adder(5);
    println!("{}", add_5(4)); //prints 9
    println!("{}", add_5(20)); //prints 25

    let i = 42;
    let capture_i = || println!("Inside closure: {i}");
    println!("Outside closure: {i}"); //  <-- New line addded
    capture_i();

    let i: String = "42".to_string(); // i is now a String, a non-copy type
    let capture_i = || println!("Inside closure: {i}");
    println!("Outside closure: {i}");
    capture_i();

    let mut animal = "fox".to_string();
    let mut capture_animal = || {
        animal.push_str("es");
    };
    capture_animal(); // mutable borrow ends here
    println!("Outside closure: {animal}"); // Ok to use animal here

    let i: String = "42".to_string();
    let capture_i = || println!("Inside closure: {i}");
    println!("Outside closure: {i}");
    capture_i();

    let i: String = "42".to_string();
    let i_ref = &i;
    let capture_i = move || println!("{i_ref}");
    println!("{i}");
    println!("{i_ref}");
    capture_i();

    let mut pluralize_fox = create_pluralizer("fox".to_string());
    pluralize_fox();

    let i = 42;
    // Both capture_nothing and capture_i implement 'Fn'
    let capture_nothing = || println!("I capture nothing");
    let capture_i = || println!("I capture i immutably: {i}");
    call_closure(capture_nothing);
    call_closure(capture_i);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn create_adder(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

fn create_pluralizer(mut animal: String) -> impl FnMut() {
    move || {
        // <- move keyword added
        animal.push_str("es");
        println!("Pluralized animal: {animal}");
    }
}

fn call_closure<C: Fn()>(c: C) {
    c();
}
