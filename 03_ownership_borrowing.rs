// LEVEL: Beginner (the key idea of Rust)
// TOPIC: Ownership, borrowing, slices
// RUN:   rustc --edition 2021 03_ownership_borrowing.rs && ./03_ownership_borrowing

// RULES:
//   1. Each value has exactly one owner.
//   2. When the owner goes out of scope, the value is dropped (freed).
//   3. You can have EITHER many `&` references OR one `&mut` reference at a time.

fn takes_ownership(s: String) {
    println!("took: {s}");
} // `s` dropped here

fn borrows(s: &String) -> usize {
    s.len() // can read, cannot modify
}

fn modifies(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    // MOVE
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves to s2
    // println!("{s1}"); // ERROR: s1 was moved
    println!("{s2}");

    // CLONE: explicit deep copy
    let a = String::from("copy me");
    let b = a.clone();
    println!("{a} / {b}");

    // Simple types (i32, bool, char, f64) implement Copy, so they are copied
    let x = 5;
    let y = x;
    println!("{x} {y}");

    // Passing to a function moves it
    let owned = String::from("moved into function");
    takes_ownership(owned);
    // println!("{owned}"); // ERROR: moved

    // BORROWING with &
    let text = String::from("borrow me");
    let len = borrows(&text);
    println!("'{text}' has length {len}"); // text still usable

    // MUTABLE borrow
    let mut greeting = String::from("hello");
    modifies(&mut greeting);
    println!("{greeting}");

    // Only one &mut at a time
    let r1 = &mut greeting;
    r1.push('!');
    // let r2 = &mut greeting; // ERROR if r1 is used afterwards
    println!("{r1}");

    // SLICES: a view into part of a collection
    let sentence = String::from("learning rust is fun");
    println!("first word: {}", first_word(&sentence));
    let part = &sentence[0..8];
    println!("slice: {part}");

    let nums = [1, 2, 3, 4, 5];
    let middle = &nums[1..4];
    println!("{:?}", middle);
}

// TRY IT: Uncomment the ERROR lines one at a time and read the compiler message.
// Then write `fn count_vowels(s: &str) -> usize`.
