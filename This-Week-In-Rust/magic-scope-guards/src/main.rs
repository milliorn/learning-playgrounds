macro_rules! blow_up {
    ($a:ident) => {
        println!("hello {}!", stringify!($a));
    };

    ($a:ident $($rest:tt)+) => {
        blow_up!($a);
        blow_up!($($rest)+);
    }
}

macro_rules! make_slow {
    () => {
        blow_up!(
            a0 b0 c0 d0 e0 f0 g0 h0 i0 j0
        );
    }
}

fn takes_closure<O, F: FnOnce() -> O>(f: F) -> O {
    make_slow!();
    let o = f();
    make_slow!();
    o
}

struct Guard;

impl Guard {
    pub fn new() -> Self {
        make_slow!();
        Self
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        make_slow!();
    }
}

fn main() {
    let a = takes_closure(|| 1u8);
    print!("{a}");
    let a = takes_closure(|| 1u16);
    print!("{a}");
    let a = takes_closure(|| 1u32);
    print!("{a}");
    let a = takes_closure(|| 1u64);
    print!("{a}");

    println!();
}
