// LEVEL: Advanced
// TOPIC: Lifetimes, closures, iterators
// RUN:   rustc --edition 2021 08_lifetimes_iterators_closures.rs && ./08_lifetimes_iterators_closures

// ---------- LIFETIMES ----------
// Lifetimes tell the compiler how long references stay valid.
// Here: the returned reference lives as long as BOTH inputs.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// A struct that holds a reference must declare a lifetime
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn first_sentence(&self) -> &str {
        self.part.split('.').next().unwrap_or("")
    }
}

// ---------- CLOSURES ----------
// Anonymous functions that can capture their environment.
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n // `move` takes ownership of n
}

fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

// ---------- CUSTOM ITERATOR ----------
struct Counter {
    n: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.n < 5 {
            self.n += 1;
            Some(self.n)
        } else {
            None
        }
    }
}

fn main() {
    // Lifetimes
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("short");
        result = longest(&s1, &s2).to_string(); // copy out before s2 dies
    }
    println!("longest: {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let ex = Excerpt { part: &novel };
    println!("first sentence: {}", ex.first_sentence());

    // Closures
    let add5 = make_adder(5);
    println!("add5(10) = {}", add5(10));
    println!("apply_twice = {}", apply_twice(add5, 1));

    let mut total = 0;
    let mut add_to_total = |x: i32| total += x; // FnMut: mutates captured var
    add_to_total(3);
    add_to_total(4);
    println!("total = {total}");

    // Iterator adapters are lazy; consumers (sum, collect, ...) run them
    let sum_even_squares: i32 = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("sum of even squares = {sum_even_squares}");

    let names = vec!["ada", "grace", "linus"];
    let shouting: Vec<String> = names.iter().map(|n| n.to_uppercase()).collect();
    println!("{:?}", shouting);

    let (short, long): (Vec<&str>, Vec<&str>) = names.iter().partition(|n| n.len() <= 3);
    println!("short {:?}, long {:?}", short, long);

    println!("any > 4 chars? {}", names.iter().any(|n| n.len() > 4));
    println!("position of grace: {:?}", names.iter().position(|&n| n == "grace"));

    let nested = vec![vec![1, 2], vec![3], vec![4, 5]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("flat {:?}", flat);

    // Using the custom iterator with built-in adapters
    let s: u32 = Counter { n: 0 }
        .zip(Counter { n: 0 }.skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("counter pipeline = {s}");

    for (i, c) in "rust".chars().enumerate() {
        print!("{i}:{c} ");
    }
    println!();
}

// TRY IT: Write a `Fibonacci` struct that implements Iterator and print the
// first 10 numbers using `.take(10)`.
