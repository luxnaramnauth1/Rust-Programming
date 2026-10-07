// LEVEL: Intermediate
// TOPIC: Result, the ? operator, custom errors
// RUN:   rustc --edition 2021 06_error_handling.rs && ./06_error_handling

use std::fmt;
use std::num::ParseIntError;

// Rust has no exceptions. Recoverable errors use Result<T, E>.
// Unrecoverable bugs use panic! (crashes the thread).

// 1. A custom error type
#[derive(Debug)]
enum AgeError {
    NotANumber(ParseIntError),
    TooOld(u32),
}

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgeError::NotANumber(e) => write!(f, "not a number: {e}"),
            AgeError::TooOld(n) => write!(f, "{n} is unrealistic"),
        }
    }
}

impl std::error::Error for AgeError {}

// 2. From lets `?` convert errors automatically
impl From<ParseIntError> for AgeError {
    fn from(e: ParseIntError) -> Self {
        AgeError::NotANumber(e)
    }
}

// 3. `?` returns early with the error if there is one
fn parse_age(input: &str) -> Result<u32, AgeError> {
    let n: u32 = input.trim().parse()?; // ParseIntError -> AgeError
    if n > 150 {
        return Err(AgeError::TooOld(n));
    }
    Ok(n)
}

// 4. Box<dyn Error>: "any error", handy in applications
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let age = parse_age("42")?;
    println!("run() parsed age {age}");
    let _bad = parse_age("abc")?; // error propagates to the caller
    println!("never reached");
    Ok(())
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    for input in ["25", "abc", "200", " 7 "] {
        match parse_age(input) {
            Ok(age) => println!("{input:?} -> valid age {age}"),
            Err(e) => println!("{input:?} -> error: {e}"),
        }
    }

    if let Err(e) = run() {
        println!("run failed: {e}");
    }

    // Handy helpers on Result / Option
    let n: i32 = "123".parse().unwrap_or(0); // default on error
    let m: i32 = "oops".parse().unwrap_or(-1);
    println!("{n} {m}");

    let doubled = divide(10.0, 4.0).map(|x| x * 2.0);
    println!("{:?} {:?}", doubled, divide(1.0, 0.0));

    // unwrap()/expect() panic on failure: use only when you are sure,
    // or in quick scripts and tests.
    let value: i32 = "99".parse().expect("should be a number");
    println!("{value}");
}

// TRY IT: Add a `Negative` variant to AgeError and handle "-5".
// (Hint: parse as i32 first.)
