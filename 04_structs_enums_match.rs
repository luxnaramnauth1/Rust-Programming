// LEVEL: Intermediate
// TOPIC: Structs, enums, pattern matching, Option
// RUN:   rustc --edition 2021 04_structs_enums_match.rs && ./04_structs_enums_match

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

impl User {
    // Associated function (constructor), called as User::new(...)
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
            active: true,
        }
    }

    // Method: takes &self
    fn greet(&self) -> String {
        format!("Hi, I'm {} ({})", self.name, self.age)
    }

    // Method that mutates
    fn birthday(&mut self) {
        self.age += 1;
    }
}

// Tuple struct
struct Point(f64, f64);

// Enum: each variant can hold different data
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

// Option<T> replaces null: Some(value) or None
fn find_user<'a>(users: &'a [User], name: &str) -> Option<&'a User> {
    users.iter().find(|u| u.name == name)
}

fn main() {
    let mut alice = User::new("Alice", 30);
    println!("{}", alice.greet());
    alice.birthday();
    println!("{:?}", alice);

    // Struct update syntax
    let bob = User {
        name: String::from("Bob"),
        ..alice.clone()
    };
    println!("{:#?}", bob); // pretty debug

    let p = Point(1.0, 2.5);
    println!("Point({}, {})", p.0, p.1);

    let shapes = vec![
        Shape::Circle(2.0),
        Shape::Rectangle { width: 3.0, height: 4.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    for s in &shapes {
        println!("{:?} -> area {:.2}", s, area(s));
    }

    // Matching with Option
    let users = vec![alice, bob];
    match find_user(&users, "Bob") {
        Some(u) => println!("found {}", u.name),
        None => println!("not found"),
    }

    // if let: when you only care about one pattern
    if let Some(u) = find_user(&users, "Zed") {
        println!("found {}", u.name);
    } else {
        println!("Zed does not exist");
    }

    // match with ranges and guards
    let n = 42;
    let kind = match n {
        0 => "zero",
        1..=9 => "single digit",
        x if x % 2 == 0 => "even, big",
        _ => "odd, big",
    };
    println!("{n}: {kind}");
}

// TRY IT: Add a `Square(f64)` variant and watch the compiler force you to
// handle it in `area`. Then add a `deactivate(&mut self)` method to User.
