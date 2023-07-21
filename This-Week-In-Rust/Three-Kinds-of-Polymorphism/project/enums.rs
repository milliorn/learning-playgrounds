enum Shape {
    Rectangle { width: f32, height: f32 },
    Triangle { side: f32 },
    Circle { radius: f32 },
}

impl Shape {
    pub fn perimeter(&self) -> f32 {
        match self {
            Shape::Rectangle { width, height } => width * 2.0 + height * 2.0,
            Shape::Triangle { side } => side * 3.0,
            Shape::Circle { radius } => radius * 2.0 * std::f32::consts::PI,
        }
    }

    pub fn area(&self) -> f32 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { side } => side * 0.5 * 3.0_f32.sqrt() / 2.0 * side,
            Shape::Circle { radius } => radius * radius * std::f32::consts::PI,
        }
    }
}

fn print_area(shape: Shape) {
    print!("{}", shape.area());
}

fn print_perimeters(shapes: Vec<Shape>) {
    for shape in shapes.iter() {
        println!("{}", shape.perimeter());
    }
}