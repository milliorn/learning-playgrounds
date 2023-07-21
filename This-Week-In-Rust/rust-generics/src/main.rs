use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, Mul},
};

fn sum_i32(i: &[i32]) -> i32 {
    i.iter().sum()
}
fn sum_f32(i: &[f32]) -> f32 {
    i.iter().sum()
}
fn sum_i8(i: &[i8]) -> i8 {
    i.iter().sum()
}

//Generic Definition of the sum of elements.
fn generic_sum<T: Sum<T> + Copy>(i: &[T]) -> T {
    i.iter().copied().sum()
}

fn add_mul<A, B>(x: A, y: A, w: B, z: B) -> B
where
    A: Add<A> + Copy,
    B: Mul<B, Output = B> + Copy,
{
    let var = x + y;
    w * z
}

fn const_generics1<const A: bool, T: Display>(i: T) {
    if A {
        println!("This is True");
    } else {
        println!("This is false");
    }
    println!("{}", i);
}
fn const_generics2<T, const N: usize>(i: [T; N])
where
    T: Debug,
{
    println!("{:?}", i);
}

fn main() {
    println!("Hello, world!");
    println!("{}", sum_i8(&[1i8, 4, 6, 7]));
    println!("{}", sum_i32(&[1, 2, 3, 4]));
    println!("{}", sum_f32(&[1.0f32, 2.0, 3.0, 4.0]));

    println!(
        "i8 Sum: {} \nu16 Sum: {} \nusize Sum: {} \nf64 Sum: {} \nf32 Sum: {} ",
        generic_sum(&[1i8, 4, 6, 7]), //The actual type is known at this line and the same for all calls.
        generic_sum(&[1u16, 5, 9, 56]),
        generic_sum(&[9usize, 34, 53, 57]),
        generic_sum(&[1.9, 4.6, 6.7, 7.9]),
        generic_sum(&[1.0f32, 2.0, 3.0, 4.0])
    );

    //This is how we specify the type generic type explicitly.
    //In this code rust won't be able to infer the type themselves.
    const_generics1::<true, &str>("string");
    const_generics1::<false, i32>(67);

    //In this code rust is able to infer both the type and value.
    const_generics2([1, 2, 3, 4]);
    const_generics2(['a', 'b', 'c']); //[i32;4]
    let m: [char; 3] = ['a', 'b', 'a']; //[char;3]
}
