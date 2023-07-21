fn main() {
    let mut s = String::from("Hello");
    let len = calculate_length(&s);
    println!("Length of {} is {}", s, len);

    change(&mut s);
    println!("{}", s);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}
