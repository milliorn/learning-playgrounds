use rand::Rng;
use std::alloc::{dealloc, Layout};
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::hash::{self, BuildHasher, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// Probably the most straightforward of generating random numbers is using the rand crate.
fn rand_rng() {
    println!("rand_rng");

    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

// fastrand is a simple and fast random number generator that uses wyhash. This hash function is also simple and fast but not cryptographically secure.
fn rand_fast_rand() {
    println!("rand_fast_rand");

    println!("Random u8: {}", fastrand::u8(..));

    // Pick an arbitrary number as seed.
    fastrand::seed(42);

    // Prints e.g. 52 every time we run.
    println!("Random u8 with seed: {}", fastrand::u8(..));
}

// Here is the poor man's random number generator which only takes the nanoseconds of the current time:

fn nanoseconds() -> Result<(), Box<dyn Error>> {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos();

    println!("nanoseconds");
    println!("Random number: {nanos}");
    Ok(())
}

// The bottom line is, if we allocate something, take the pointer to it, retrieve the address, and cast it to an integer; we will get a random number. The address in the memory is not totally random, but probably random enough to do stuff. The problem with that approach is, apart from using a memory address for a random number, the numbers are always even since they are a memory address aligned to the size of an integer. Also, the code above has memory leaks due to not handling the memory after calling Box::into_raw.
fn raw_pointers() {
    let pointer = Box::into_raw(Box::new(42));

    println!("raw_pointers");
    println!("Random number: {}", pointer as usize);

    // So we can edit the code as follows for manual cleanup by explicitly running the destructor and deallocating the memory:
    unsafe {
        std::ptr::drop_in_place(pointer);
        dealloc(pointer as *mut u8, Layout::new::<u32>());
    }
}

// Simply, we can use the RandomState of the standard library's HashMap implementation and generate a random number from the hasher:
fn hasher_func() {
    let hasher = RandomState::new().build_hasher();
    println!("Hasher function");
    println!("Random number: {}", hasher.finish())
}

// It probably makes more sense to use these random values as seed rather than random number. For example, we can have the following function for generating a random seed and combine it with a suggestion by matklad:
fn random_seed() -> u64 {
    RandomState::new().build_hasher().finish()
}

fn random_numbers() -> impl Iterator<Item = u32> {
    let mut random = 92u32;
    std::iter::repeat_with(move || {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        random
    })
}

fn main() {
    println!("Hello, world!");
    rand_rng();
    rand_fast_rand();
    nanoseconds();
    raw_pointers();
    hasher_func();

    random_seed();
    random_numbers();

    println!("Let's just use a raw pointer via *const T:");
    let num = 42u8;
    let address = &num as *const u8;
    println!("{}", address as usize);

    // ASLR (address space layout randomization) implementations tend to have weaknesses that manifest in ways that might not be obvious just looking at the scrambled output.
    dbg![main as *const u8];
    dbg![main as *const u8 as usize % 4096];
}
