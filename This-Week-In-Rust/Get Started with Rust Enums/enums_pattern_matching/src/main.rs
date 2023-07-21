/* We can use enums to recreate a Bool data type with two variants: True and False. */
#[derive(Debug, Eq, PartialEq)]
enum Bool {
    False,
    True,
}

impl Bool {
    fn neg(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
        }
    }
}

/* Suppose we want to create a function called neg that returns the opposite of the boolean that we provide. This is how we can do it in Rust using pattern matching.*/
fn neg(value: Bool) -> Bool {
    match value {
        Bool::True => Bool::False,
        Bool::False => Bool::True,
    }
}

/* Enum variants are like structs: they can contain fields, which can be unnamed or named. */
#[derive(Debug)]
enum HealthBar {
    Alive { life: i8 },
    Dead,
}

/* Enums enable you to build clearer types and decrease the number of illegal states that your data can take. */
enum Player {
    Alive { life: i8 },
    KnockedOut { life: i8, turns_to_wait: i8 },
    Dead,
}

/* The result of a match statement can be either an expression or a statement.*/
fn print_value(value: Bool) {
    match value {
        Bool::True => println!("Clearly true!"),
        Bool::False => println!("Unfortunately false!"),
    }
}

/* If you want the result to be more than a single statement or expression, you can use curly braces*/
fn print_and_return_value(value: Bool) -> Bool {
    match value {
        Bool::True => {
            println!("Clearly true!");
            Bool::True
        }
        Bool::False => {
            println!("Unfortunately false!");
            Bool::False
        }
    }
}

/* While this basic case highly resembles a case/switch statement, pattern matching can do much more. We can use pattern matching to assign values while choosing the code path to execute. This enables us to easily work with nested values.*/
fn take_five(player: Player) -> Player {
    match player {
        Player::Alive { life: life } => Player::Alive { life: life - 5 },
        Player::KnockedOut {
            life: life,
            turns_to_wait: turns_to_wait,
        } => Player::KnockedOut {
            life: life - 5,
            turns_to_wait: turns_to_wait,
        },
        Player::Dead => Player::Dead,
    }
}

/* If you put a wildcard (marked by underscore) in a pattern, it will match anything. It can be used at the end of a match statement to handle all the remaining cases.*/
fn take_five_advanced(player: Player) -> Player {
    match player {
        Player::Alive { life } if life > 5 => Player::Alive { life: (life - 5) },
        Player::KnockedOut {
            life,
            turns_to_wait,
        } if life > 5 => Player::KnockedOut {
            life: (life - 5),
            turns_to_wait,
        },
        _ => Player::Dead,
    }
}

#[derive(Debug, Clone)]
struct DivideByZero; // custom error type

fn safe_division_result(a: i32, b: i32) -> (Result<i32, DivideByZero>) {
    match b {
        0 => Err(DivideByZero),
        _ => Ok(a / b),
    }
}

fn main() {
    println!("Hello, world!");

    /* We can get a value of this type by constructing it with one of the two variants. */
    let is_true = Bool::True;
    let is_false = Bool::False;
    println!("{:?}", is_true);
    println!("{:?}", is_false);

    /* For example, after deriving Debug and PartialEq, we can now print and compare values of Bool.*/
    let are_same = is_true == is_false; // false
    println!("{:?}", are_same);

    let health_score = HealthBar::Alive { life: 8 };
    let is_dead = HealthBar::Dead;
    println!("{:?}", health_score);
    println!("{:?}", is_dead);

    /* Store the name of the state in a string. If the character is “dead”, their life should be 0. And if they are “knocked out”, they should have some turns to wait until they can act. */
    let dead_player = Player::Dead;
    let knocked_out_player = Player::KnockedOut {
        life: 50,
        turns_to_wait: 3,
    };

    /* Two enums you will frequently encounter in Rust are Option, Rust’s safe alternative to null, and Result, Rust’s safe alternative to exceptions. In fact, Rust doesn’t let you compile a line of code that divides by zero. But you can still do it in multiple other ways.*/
    println!("Choose what to divide 4 with!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let divisor: i32 = input.trim().parse().unwrap();

    let result = 4 / divisor;

    let possible = safe_division(4, 0); // None
}
