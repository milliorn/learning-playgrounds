struct User {
    name: String,
    city: String,
    country: String,
}

fn print_user(user: &User) {
    let User {
        name,
        city,
        country,
    } = user;

    println!("User {} is from {}, {}", name, city, country);
}

fn print_user2(user: &User) {
    let User {
        name: fullname,
        city: metro,
        country: nation,
    } = user;

    println!("User {} is from {}, {}", fullname, metro, nation);
}

fn my_function(data: &(u32, &str)) {
    let (my_num, my_str) = data;
    println!("my_num: {}, my_str: {}", my_num, my_str);
}

fn city_name(user: &User) -> String {
    let User {
        name: _,
        city,
        country,
    } = user;
    format!("{}, {}", city, country)
}

fn main() {
    println!("Hello, world!");

    let data = (4, "OK");
    my_function(&data);

    let user = User {
        name: "Time".to_string(),
        city: "Ottawa".to_string(),
        country: "Canada".to_string(),
    };

    print_user(&user);
    print_user2(&user);
    println!("{}", city_name(&user));

    let my_tuple = (4, "foo", false);

    let (num, _, truthy) = my_tuple;

    println!("{} {}", num, truthy);
}
