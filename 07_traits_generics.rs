// LEVEL: Intermediate / Advanced
// TOPIC: Traits, generics, trait objects
// RUN:   rustc --edition 2021 07_traits_generics.rs && ./07_traits_generics

use std::fmt::Display;

// A trait defines shared behavior (like an interface)
trait Shape {
    fn area(&self) -> f64;

    // Default implementation
    fn describe(&self) -> String {
        format!("a shape with area {:.2}", self.area())
    }
}

struct Circle(f64);
struct Square(f64);

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.0 * self.0
    }
    fn describe(&self) -> String {
        format!("a circle with radius {}", self.0)
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

// GENERICS with trait bounds: works for any T that can be compared and copied
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut max = items[0];
    for &item in items {
        if item > max {
            max = item;
        }
    }
    max
}

// `impl Trait` in argument position (static dispatch)
fn print_area(shape: &impl Shape) {
    println!("{} -> {:.2}", shape.describe(), shape.area());
}

// `where` clause for readable bounds
fn show_all<T>(items: &[T])
where
    T: Display,
{
    let parts: Vec<String> = items.iter().map(|i| i.to_string()).collect();
    println!("[{}]", parts.join(", "));
}

// Generic struct with a generic impl
struct Pair<T> {
    a: T,
    b: T,
}

impl<T: PartialOrd + Display> Pair<T> {
    fn show_largest(&self) {
        if self.a >= self.b {
            println!("largest is a = {}", self.a);
        } else {
            println!("largest is b = {}", self.b);
        }
    }
}

// Returning a closure/iterator-like type with impl Trait
fn make_shape(big: bool) -> Box<dyn Shape> {
    // Box<dyn Trait> = trait object (dynamic dispatch), needed when
    // different concrete types can be returned.
    if big { Box::new(Circle(10.0)) } else { Box::new(Square(2.0)) }
}

fn main() {
    println!("{}", largest(&[3, 9, 2]));
    println!("{}", largest(&[1.5, 0.2]));
    println!("{}", largest(&['a', 'z', 'm']));

    print_area(&Circle(1.5));
    print_area(&Square(3.0));

    // Heterogeneous collection via trait objects
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle(1.0)),
        Box::new(Square(4.0)),
        make_shape(true),
    ];
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("total area = {total:.2}");

    show_all(&[1, 2, 3]);
    show_all(&["x", "y"]);

    Pair { a: 10, b: 20 }.show_largest();
    Pair { a: "pear", b: "apple" }.show_largest();
}

// TRY IT: Add a `Triangle` that implements Shape, and a generic
// `fn total_area<T: Shape>(items: &[T]) -> f64`.
