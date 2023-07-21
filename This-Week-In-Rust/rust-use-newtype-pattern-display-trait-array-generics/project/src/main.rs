use std::fmt::Display;
use std::ptr::eq;

struct Planet {
    name: String,
    surface_area: i64,
    polar_radius: f64,
}

impl Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-> {}:\n\tSurface: {} km2\n\tPolar radius: {} km",
            self.name, self.surface_area, self.polar_radius
        )
    }
}

struct Wrap<T, const N: usize>([T; N]);

impl<T, const N: usize> Display for Wrap<T, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let separator = ", ";
        let mut s = "Array -> ".to_string();

        for element in &self.0 {
            s.push_str(&element.to_string());
            if !eq(element, self.0.last().unwrap()) {
                s.push_str(&separator);
            }
        }
        write!(f, "{}", s)
    }
}

fn main() {
    let planet = Planet {
        name: "Earth".to_string(),
        surface_area: 510072000,
        polar_radius: 6356.752,
    };

    println!("{}", planet);

    let my_array = [1, 3, 1];

    let my_wrap = Wrap(my_array);
    println!("{}", my_wrap);

    let my_array_f = [1.5, 2.5, 3.5, 4.5, 5.2, 6.75, 8.90, 8.90, 8.90];
    let my_array_c = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'j', 'j'];
    let my_array_s = [
        "Hello".to_string(),
        "World!".to_string(),
        "from Rust!".to_string(),
    ];

    let my_wrap = Wrap(my_array_f);
    println!("{}", my_wrap);

    let my_wrap = Wrap(my_array_c);
    println!("{}", my_wrap);

    let my_wrap = Wrap(my_array_s);
    println!("{}", my_wrap);
}
