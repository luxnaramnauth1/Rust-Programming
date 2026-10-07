// LEVEL: Beginner
// TOPIC: Control flow and functions
// RUN:   rustc --edition 2021 02_control_flow_functions.rs && ./02_control_flow_functions

// Functions: parameter types are required; last expression (no `;`) is returned
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

fn main() {
    // if / else if / else is an expression
    let n = 7;
    if n < 0 {
        println!("negative");
    } else if n == 0 {
        println!("zero");
    } else {
        println!("positive");
    }
    let label = if is_even(n) { "even" } else { "odd" };
    println!("{n} is {label}");

    // loop: infinite until break; break can return a value
    let mut tries = 0;
    let result = loop {
        tries += 1;
        if tries == 3 {
            break tries * 10;
        }
    };
    println!("result = {result}");

    // while
    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}...");
        countdown -= 1;
    }
    println!("Liftoff!");

    // for with ranges: 1..5 excludes 5, 1..=5 includes 5
    for i in 1..=5 {
        println!("{i} squared = {}", i * i);
    }

    // for over a collection
    let fruits = ["apple", "banana", "cherry"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{index}: {fruit}");
    }

    println!("add(2, 3) = {}", add(2, 3));
    for i in 1..=15 {
        print!("{} ", fizzbuzz(i));
    }
    println!();
}

// TRY IT: Write a function `factorial(n: u64) -> u64` using a `for` loop.
// Then write `is_prime(n: u32) -> bool`.
