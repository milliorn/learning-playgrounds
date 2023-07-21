use futures::{stream, StreamExt};
use rand::{thread_rng, Rng};
use std::time::Duration;
use std::{collections::HashMap, fmt, iter::FromIterator, vec};

/* Rust is primarily an expression language: most chunks of code are producing values. */
fn add_one(x: u64) -> u64 {
    x + 1
}

/*
Rust's variables are immutable by default.
You have to be explicit when you want a variable to be mutable.
*/
fn push_forty_two(v: &mut Vec<u64>) {
    v.push(42)
}

/* Functions are first-class citizens */
fn apply<F: Fn(&str)>(x: &[&str], f: F) {
    for elem in x {
        f(&elem)
    }
}

/*
An Iterator is an object that enables developers to traverse collections.
Iterators can be obtained from most of the collections of the standard library.
 */
fn vector() {
    let v = vec![1, 2, 3];

    for x in v.into_iter() {
        println!("{}", x);
    }
}

/*
Then, iter which provides a borrowed iterator.
Here key and value variables are references (&String in this case).
 */
fn hashmap() {
    let mut h = HashMap::new();
    h.insert(String::from("Hello"), String::from("World"));

    for (key, value) in h.iter() {
        println!("{}: {}", key, value);
    }
}

/* Since version 1.53 (released on June 17, 2021), iterators can also be obtained from arrays: */
fn array() {
    let a = [1, 2, 3];

    for x in a.iter() {
        println!("{}", x)
    }
}

/* Iterators are lazy: they won't do anything if they are not consumed. */
fn for_each() {
    let v = vec!["Hello", "World", "!"].into_iter();

    v.for_each(|word| {
        println!("{}", word);
    });
}

/* collect can be used to transform an iterator into a collection: */
fn collect() {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();

    let _: Vec<u64> = x.collect();
}

/* Conversely, you can obtain a HashMap, BTreeMap, or other collections, using from_iter: */
fn from_iter() {
    let x = vec![(1, 2), (3, 4), (5, 6)].into_iter();

    let _: HashMap<u64, u64> = HashMap::from_iter(x);
}

/*
reduce accumulates over an iterator by applying a closure:
 */
fn reduce() {
    let values = vec![1, 2, 3, 4, 5].into_iter();
    let _sum = values.reduce(|acc, x| acc + x);

    for x in _sum.iter() {
        println!("{}", x)
    }
}

/* fold is like reduce but can return an accumulator of a different type than the items of the iterator: */
fn fold() {
    let values = vec!["Hello", "World", "!"].into_iter();
    let _sentence = values.fold(String::new(), |acc, x| acc + x);
}

/* Combinators. First, one of the most famous, and available in almost all languages: filter: */
fn filter() {
    let v = vec![-1, 2, -3, 4, 5].into_iter();
    let _positive_numbers: Vec<i32> = v.filter(|x: &i32| x.is_positive()).collect();
}

/* inspect can be used to... inspect the values flowing through an iterator: */
fn inspect() {
    let v = vec![-1, 2, -3, 4, 5].into_iter();

    let _positive_numbers: Vec<i32> = v
        .inspect(|x| println!("Before filter: {}", x))
        .filter(|x: &i32| x.is_positive())
        .inspect(|x| println!("After filter: {}", x))
        .collect();
}
/* map is used to convert an the items of an iterator from one type to another: */
fn map() {
    let v = vec!["Hello", "World", "!"].into_iter();

    let _w: Vec<String> = v.map(String::from).collect();
}

/*
filter_map is kind of like chainng map and filter.
It has the advantage of dealing with Option instead of bool:
*/
fn filter_map() {
    let v = vec!["Hello", "World", "!"].into_iter();

    let w: Vec<String> = v
        .filter_map(|x| {
            if x.len() > 2 {
                Some(String::from(x))
            } else {
                None
            }
        })
        .collect();

    assert_eq!(w, vec!["Hello".to_string(), "World".to_string()]);
}

/* chain merges two iterators: */
fn chain() {
    let x = vec![1, 2, 3, 4, 5].into_iter();
    let y = vec![6, 7, 8, 9, 10].into_iter();

    let z: Vec<u64> = x.chain(y).collect();
    assert_eq!(z.len(), 10);
}

/*
Enums are certainly the favorite Rust's feature of new Rustaceans because they are the foundations of Result and Option.
They allow us to express all the invariants of the domain and check at compile time that all cases are covered.
 */
enum Platform {
    Linux,
    MacOs,
    Windows,
    Unknown,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Platform::Linux => write!(f, "Linux"),
            Platform::MacOs => write!(f, "macOS"),
            Platform::Windows => todo!(),
            Platform::Unknown => todo!(),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let a: u64 = add_one(1);
    println!("{}", a);

    let mut v = Vec::new();
    push_forty_two(&mut v);
    println!("{:?}", v);

    let first_class = vec!["hello", "world"];
    apply(&first_class, |x| println!("{}", x));

    vector();
    hashmap();
    array();
    for_each();
    collect();
    from_iter();
    reduce();
    fold();
    filter();
    inspect();
    map();
    filter_map();
    chain();
    /*
    Combinators are methods that ease the manipulation of some type T.
    They favor a functional (method chaining) style of code.
    */
    let sum: u64 = vec![1, 2, 3].into_iter().map(|x| x * x).sum();
    println!("{}", sum);

    #[tokio::main(flavor = "multi_thread")]
    async fn main() {
        stream::iter(0..200u64)
            .for_each_concurrent(20, |number| async move {
                let mut rng = thread_rng();
                let sleep_ms: u64 = rng.gen_range(0..20);
                tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
                println!("{}", number);
            })
            .await;
    }
}
