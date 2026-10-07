// LEVEL: Advanced
// TOPIC: Threads, channels, shared state
// RUN:   rustc --edition 2021 10_concurrency.rs && ./10_concurrency

use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    // ---------- 1. Spawn and join ----------
    let handle = thread::spawn(|| {
        let mut s = 0;
        for i in 1..=100 {
            s += i;
        }
        s
    });
    let result = handle.join().unwrap(); // wait and get the return value
    println!("thread computed {result}");

    // `move` transfers ownership of data into the thread
    let data = vec![1, 2, 3];
    let h = thread::spawn(move || {
        println!("thread sees {:?}", data);
    });
    h.join().unwrap();

    // ---------- 2. Shared state: Arc<Mutex<T>> ----------
    // Arc  = atomic reference counting (thread-safe Rc)
    // Mutex = only one thread at a time can access the data
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("counter = {} (expected 8000)", *counter.lock().unwrap());

    // ---------- 3. RwLock: many readers OR one writer ----------
    let config = Arc::new(RwLock::new(String::from("v1")));
    {
        let r1 = config.read().unwrap();
        let r2 = config.read().unwrap();
        println!("two readers: {} {}", *r1, *r2);
    }
    *config.write().unwrap() = String::from("v2");
    println!("after write: {}", config.read().unwrap());

    // ---------- 4. Message passing with channels ----------
    // "Do not communicate by sharing memory; share memory by communicating."
    let (tx, rx) = mpsc::channel();
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(10 * (3 - id)));
            tx.send(format!("worker {id} done")).unwrap();
        });
    }
    drop(tx); // close the original sender so the receiver loop can end
    for msg in rx {
        println!("received: {msg}");
    }

    // ---------- 5. Scoped threads: borrow local data, no Arc needed ----------
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = numbers.split_at(3);
    let (a, b) = thread::scope(|s| {
        let ha = s.spawn(|| left.iter().sum::<i32>());
        let hb = s.spawn(|| right.iter().sum::<i32>());
        (ha.join().unwrap(), hb.join().unwrap())
    });
    println!("left sum {a}, right sum {b}");
}

// NOTES:
//  - The types Send and Sync make data races a COMPILE error.
//  - For data parallelism over iterators, see the `rayon` crate.
//  - For async I/O, see `tokio` (async/await).
// TRY IT: Split a Vec of 1,000,000 numbers across 4 threads and sum it.
