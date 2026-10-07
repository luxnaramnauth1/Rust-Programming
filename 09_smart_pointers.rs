// LEVEL: Advanced
// TOPIC: Box, Rc, RefCell
// RUN:   rustc --edition 2021 09_smart_pointers.rs && ./09_smart_pointers

use std::cell::RefCell;
use std::rc::Rc;

// ---------- Box<T>: heap allocation, single owner ----------
// Needed for recursive types, whose size must be known at compile time.
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

fn sum(list: &List) -> i32 {
    match list {
        Cons(value, rest) => value + sum(rest),
        Nil => 0,
    }
}

// ---------- Rc<T>: shared ownership (single thread) ----------
// ---------- RefCell<T>: interior mutability (checked at runtime) ----------
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

struct Counter {
    hits: RefCell<u32>,
}

impl Counter {
    // &self (immutable) but we can still mutate through RefCell
    fn hit(&self) {
        *self.hits.borrow_mut() += 1;
    }
}

fn main() {
    // Box
    let boxed = Box::new(42);
    println!("boxed = {}", *boxed);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("sum of list = {}", sum(&list));

    // Rc: many owners, freed when the last one is dropped
    let shared = Rc::new(String::from("shared data"));
    println!("count after creation = {}", Rc::strong_count(&shared));
    let a = Rc::clone(&shared);
    {
        let _b = Rc::clone(&shared);
        println!("count with 3 owners = {}", Rc::strong_count(&shared));
    }
    println!("count after inner scope = {}", Rc::strong_count(&shared));
    println!("a = {a}");

    // Rc<RefCell<T>>: shared AND mutable
    let data = Rc::new(RefCell::new(vec![1, 2, 3]));
    let other = Rc::clone(&data);
    other.borrow_mut().push(4);
    data.borrow_mut().push(5);
    println!("data = {:?}", data.borrow());

    // RefCell panics at runtime if borrow rules are broken:
    // let _x = data.borrow_mut();
    // let _y = data.borrow_mut(); // PANIC: already borrowed

    let c = Counter { hits: RefCell::new(0) };
    c.hit();
    c.hit();
    println!("hits = {}", c.hits.borrow());

    // A tree where nodes are shared
    let leaf = Rc::new(Node { value: 3, children: RefCell::new(vec![]) });
    let branch = Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    };
    println!(
        "branch {} has child {} (leaf owners: {})",
        branch.value,
        branch.children.borrow()[0].value,
        Rc::strong_count(&leaf)
    );
}

// NOTES:
//  - Use Arc + Mutex instead of Rc + RefCell across threads (see file 10).
//  - Rc cycles leak memory; use Weak<T> to break cycles.
// TRY IT: Build a binary tree enum with Box and write a function that sums it.
