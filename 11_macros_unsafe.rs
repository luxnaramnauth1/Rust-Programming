// LEVEL: Advanced
// TOPIC: Declarative macros, unsafe Rust
// RUN:   rustc --edition 2021 11_macros_unsafe.rs && ./11_macros_unsafe

// ---------- MACROS (macro_rules!) ----------
// Macros generate code at compile time and can take a variable number of args.

macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

// Repetition: $( ... ),* means "zero or more, separated by commas"
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {{
        let mut v = Vec::new();
        $( v.push($x); )*
        v
    }};
}

// Macro that generates a function
macro_rules! make_greeter {
    ($name:ident, $greeting:expr) => {
        fn $name(who: &str) -> String {
            format!("{}, {}!", $greeting, who)
        }
    };
}

make_greeter!(say_hello, "Hello");
make_greeter!(say_bye, "Goodbye");

// Macro with multiple arms
macro_rules! max_of {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => {{
        let a = $x;
        let b = max_of!($($rest),+);
        if a > b { a } else { b }
    }};
}

// ---------- UNSAFE ----------
// `unsafe` does not turn off the borrow checker. It unlocks 5 extra powers:
//   1. dereference raw pointers   2. call unsafe functions
//   3. access/modify mutable statics  4. implement unsafe traits
//   5. access union fields
// You promise the compiler that YOU uphold the safety rules.

static mut GLOBAL_COUNTER: u32 = 0;

unsafe fn dangerous_add(ptr: *mut i32, amount: i32) {
    *ptr += amount;
}

// Best practice: wrap unsafe code in a safe API
fn safe_swap(a: &mut i32, b: &mut i32) {
    let pa: *mut i32 = a;
    let pb: *mut i32 = b;
    unsafe {
        let tmp = *pa;
        *pa = *pb;
        *pb = tmp;
    }
}

fn main() {
    println!("square!(7) = {}", square!(7));
    println!("my_vec! = {:?}", my_vec![1, 2, 3, 4]);
    println!("{}", say_hello("Rust"));
    println!("{}", say_bye("C++"));
    println!("max_of! = {}", max_of!(3, 9, 4, 1));

    // Raw pointers
    let mut n = 10;
    let p = &mut n as *mut i32;
    unsafe {
        *p += 5;
        dangerous_add(p, 100);
    }
    println!("n = {n}");

    let (mut a, mut b) = (1, 2);
    safe_swap(&mut a, &mut b);
    println!("a={a}, b={b}");

    unsafe {
        GLOBAL_COUNTER += 1;
        let value = GLOBAL_COUNTER; // copy out before printing
        println!("GLOBAL_COUNTER = {value}");
    }
}

// FFI (calling C) looks like this in the 2021 edition:
//
//   extern "C" { fn abs(x: i32) -> i32; }
//   fn main() { unsafe { println!("{}", abs(-3)); } }
//
// (In edition 2024 you write `unsafe extern "C"`.)
// Procedural macros (derive, attribute) live in their own crate using
// `syn` and `quote`: read the Rust Book chapter "Macros" to go deeper.
//
// TRY IT: Write a macro `hashmap!{ "a" => 1, "b" => 2 }` that builds a HashMap.
