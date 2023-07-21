use auto_enums::auto_enum;
use either::Either;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MyStruct(u32, u16);

fn foo0() -> MyStruct {
    MyStruct(100, 12)
}

fn foo1() -> (u32, u16) {
    (100, 12)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum MyEnum {
    Left(u32),
    Right(u16),
}

fn foo3(cond: bool) -> MyEnum {
    if cond {
        MyEnum::Left(100)
    } else {
        MyEnum::Right(12)
    }
}

fn fooEither(cond: bool) -> Either<u32, u16> {
    if cond {
        Either::Left(100)
    } else {
        Either::Right(12)
    }
}

#[auto_enum(Debug)]
fn foo(cond: bool) -> impl Debug {
    if cond {
        100u32
    } else {
        12u16
    }
}

fn main() {
    println!("Hello, world!");
}
