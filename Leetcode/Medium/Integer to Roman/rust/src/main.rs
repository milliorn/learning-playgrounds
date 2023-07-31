struct Solution {}

static ROMAN: &'static [(char, i32)] = &[
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000),
];
static ROMAN_PAIRS: &'static [(&'static str, i32)] = &[
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut out = String::new();
        let mut n = num;

        for &(key, value) in ROMAN_PAIRS.iter() {
            while n >= value {
                n -= value;
                out.push_str(key);
            }
        }
        out
    }
}

fn main() {
    println!("Hello, world!");
}
