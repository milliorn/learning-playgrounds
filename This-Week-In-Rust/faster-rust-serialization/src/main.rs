use serde::Serialize;
use serde_json::value::Serializer as MySerializer;
use std::fmt::{self, Display};

fn main() {
    let name = Name {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
    };

    println!("My name is {}", name);
}

struct Name {
    first_name: String,
    last_name: String,
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self)
    }
}

fn naive(names: &[Name]) -> serde_json::Result<String> {
    let full_names = names
        .iter()
        .map(|name| name.to_string())
        .collect::<Vec<_>>();

    serde_json::to_string(&full_names)
}

fn manual_serialize(names: &[Name]) -> serde_json::Result<String> {
    serde_json::to_string(&names)
}
