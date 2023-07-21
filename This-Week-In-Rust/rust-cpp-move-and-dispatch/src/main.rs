struct Struct;

impl Drop for Struct {
    fn drop(&mut self) {
        println!("dropped")
    }
}

trait Trait {
    fn f(&self);
}

struct Impl;

impl Trait for Impl {
    fn f(&self) {
        println!("f from Impl");
    }
}

fn main() {
    println!("Hello, world!");

    let a: Struct = Struct;
    let _b: Struct = a;

    let a: Impl = Impl;
    let b: &dyn Trait = &a;
    b.f();
    println!("Size of Impl is {}", std::mem::size_of::<Impl>());
}
