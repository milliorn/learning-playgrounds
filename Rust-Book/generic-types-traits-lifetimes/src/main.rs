fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 50, 25, 100, 65];

    let mut large = &number_list[0];

    for number in &number_list {
        if number > large {
            large = number;
        }
    }

    println!("The largest number is {}", large);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut larger = &number_list[0];

    for number in &number_list {
        if number > larger {
            larger = number;
        }
    }

    println!("The largest number is {}", larger);

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
