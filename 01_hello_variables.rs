// LEVEL: Beginner
// TOPIC: Hello world, variables, types
// RUN:   rustc --edition 2021 01_hello_variables.rs && ./01_hello_variables

fn main() {
    // println! is a macro (note the !)
    println!("Hello, Rust!");

    // Variables are immutable by default
    let x = 5;
    println!("x = {x}");
    // x = 6; // ERROR: cannot assign twice to immutable variable

    // Use `mut` to allow change
    let mut counter = 0;
    counter += 1;
    println!("counter = {counter}");

    // Shadowing: a new variable with the same name (can even change type)
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {spaces}");

    // Constants: always immutable, type required, UPPER_CASE
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {MAX_POINTS}");

    // Scalar types
    let age: u8 = 30; // unsigned 8-bit (0..=255)
    let temperature: i32 = -12; // signed 32-bit
    let pi: f64 = 3.14159; // 64-bit float
    let is_rust_fun: bool = true;
    let letter: char = 'R'; // char is a Unicode scalar, 4 bytes
    println!("{age} {temperature} {pi} {is_rust_fun} {letter}");

    // Compound types
    let tuple: (i32, f64, &str) = (500, 6.4, "hi");
    let (a, b, c) = tuple; // destructuring
    println!("{a} {b} {c} | first = {}", tuple.0);

    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // fixed size array
    println!("first = {}, len = {}", numbers[0], numbers.len());
    println!("{:?}", numbers); // {:?} = Debug formatting

    // Type conversion must be explicit
    let big: i64 = 1_000;
    let small: i32 = big as i32;
    println!("small = {small}");
}

// TRY IT: Create variables for your name, age and favorite number and print
// a sentence with them. Then try to modify one without `mut` and read the error.
