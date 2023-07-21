use parking_lot::Mutex;
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Protection {
    Secure(SecureVersion),
    Insecure,
}

#[derive(Clone, Copy, Debug)]
pub enum SecureVersion {
    V1,
    V2,
    V2_1,
}

fn is_good() -> bool {
    true
}

fn process(prot: &Protection) {
    match prot {
        Protection::Secure(version) => {
            println!("Hacker-safe thanks to protocol v{version:?}");
        }
        Protection::Insecure => {
            println!("Come on in");
        }
    }
}

struct NoisyDrop;

impl Drop for NoisyDrop {
    fn drop(&mut self) {
        println!("dropping!")
    }
}

#[derive(Debug)]
struct Foobar(i64);

impl Foobar {
    fn negated(&self) -> &i64 {
        &self.0
    }

    fn get(&self) -> &i64 {
        &self.0
    }
}

#[derive(Default)]
struct State {
    value: u64,
}

impl State {
    fn foo(&mut self) {
        self.value += 4;
        self.bar();
        println!("exiting foo, value = {}", self.value);
    }

    fn bar(&mut self) {
        if self.value > 10 {
            self.value = 0
        }
    }
}

enum Node<T> {
    Left(T),
    Right(T),
}

impl<T> Node<T> {
    fn get(&self) -> &T {
        match self {
            Self::Left(x) => x,
            Self::Right(x) => x,
        }
    }
}

fn main() {
    let good = match is_good() {
        true => "It is good",
        false => "It isn't good, yet",
    };
    println!("{good}");

    let results = match rand::thread_rng().gen_range(0..=10) {
        10 => "Overwhelming victory",
        5.. => "Victory",
        _ => "Defeat",
    };
    println!("{results}");

    let prot = Protection::Secure(SecureVersion::V2_1);
    process(&prot);
    process(&prot);

    let counter = Mutex::new(0_u64);

    crossbeam::scope(|s| {
        for _ in 0..3 {
            s.spawn(|_| {
                for _ in 0..100_000 {
                    *counter.lock() += 1;
                }
            });
        }
    })
    .unwrap();

    /*
        let counter = counter.into_inner();
        println!("final count: {counter}");
    */
    let mut guard = counter.lock();

    let mutable_ref: &mut u64 = &mut guard;
    *mutable_ref = 42;

    let immutable_ref: &u64 = &guard;
    dbg!(immutable_ref);

    let nd = NoisyDrop;
    println!("before drop...");
    drop(nd);
    println!("after drop!");

    let g = Foobar(72);
    println!("{g:?}");

    let c = g.negated();
    println!("{c}");

    let f = Foobar(134);
    let d = f.get();
    //drop(f)
    println!("d = {d}");

    let s: Mutex<State> = Default::default();
    for _ in 0..5 {
        s.lock().foo();
    }

    let a = Mutex::new(Node::Left(23));
    let b = Mutex::new(Node::Right(47));

    fn inspect(n: &Mutex<Node<u64>>) {
        let msg = {
            let tmp1 = n.lock();
            let tmpl2 = tmp1.get();
            match tmpl2 {
                0 => "zero",
                _ => "non-zero",
            }
        };
        println!("{msg}")
    }
    inspect(&a);
    inspect(&b);
}
