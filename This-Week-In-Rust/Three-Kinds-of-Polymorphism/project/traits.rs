trait Shape {
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
}

struct Rectangle {
    pub width: f32,
    pub height: f32,
}

struct Triangle {
    pub side: f32,
}

struct Circle {
    pub radius: f32,
}

impl Shape for Rectangle {
    fn perimeter(&self) -> f32 {
        self.width * 2.0 + self.height * 2.0
    }
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Shape for Triangle {
    fn perimeter(&self) -> f32 {
        self.side * 3.0
    }
    fn area(&self) -> f32 {
        self.side * 0.5 * 3.0_f32.sqrt() / 2.0 * self.side
    }
}

impl Shape for Circle {
    fn perimeter(&self) -> f32 {
        self.radius * 2.0 * std::f32::consts::PI
    }
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

fn print_area<S: Shape>(shape: S) {
    println!("{}", shape.area());
}

fn print_area(shape: &dyn Shape) {
    println!("{}", shape.area());
}

fn print_perimeters<S: Shape>(shapes: Vec<S>) {
    for shape in shapes.iter() {
        println!("{}", shape.perimeter());
    }
}

fn print_perimeters(shapes: Vec<&dyn Shape>) {
    for shape in shapes.iter() {
        println!("{}", shape.perimeter());
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    let circle = Circle { radius: 1.0 };

    print_area(&rectangle); // ✅
    print_area(&circle); // ✅

    print_perimeters(vec![&rectangle, &circle]); // ✅
}
