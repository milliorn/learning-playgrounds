use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::time::Duration;
use std::{collections::VecDeque, sync::Condvar, sync::Mutex};
use std::{thread, vec};

trait Meower {
    fn meow(&self);
}

struct Cat {}

impl Meower for Cat {
    fn meow(&self) {
        println!("meow");
    }
}

fn do_the_meow(meower: &impl Meower) {
    meower.meow();
}

/// Example taken from the Rustonomicon
fn item_at_index(idx: usize, arr: &[u8]) -> Option<u8> {
    if idx < arr.len() {
        unsafe { Some(*arr.get_unchecked(idx)) }
    } else {
        None
    }
}

struct Example<T> {
    value: T,
}

impl<T> Deref for Example<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Example<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub struct Foo {
    x: u64,
}

impl Foo {
    /// Any type that borrows an instance of Foo can
    /// call this method, as it only requires a reference to Foo.
    pub fn total(&self) -> u64 {
        self.x
    }
    /// Only exclusive references to instances of Foo
    /// can call this method, as it requires Foo to be mutable.
    pub fn increase(&mut self) {
        self.x += 1;
    }
}

fn main() {
    println!("Hello, world!");

    let queue = Mutex::new(VecDeque::new());

    let foo = Foo { x: 10 };
    println!("{}", foo.total()); // WORKS.

    let mut x = Example { value: 'a' };
    *x = 'b';

    let y = assert_eq!('b', x.value);
    println!("{}", x.value);

    let items = vec![1, 2, 3, 4, 5];

    let items = vec![1, 2, 3, 4, 5];

    for item in items.into_iter() {
        println!("{}", item);
    }

    thread::scope(|s| {
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    })
}
