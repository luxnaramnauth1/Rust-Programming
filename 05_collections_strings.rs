// LEVEL: Intermediate
// TOPIC: Vec, HashMap, HashSet, Strings
// RUN:   rustc --edition 2021 05_collections_strings.rs && ./05_collections_strings

use std::collections::{HashMap, HashSet};

fn main() {
    // ---------- Vec<T>: growable array ----------
    let mut v = vec![3, 1, 4];
    v.push(1);
    v.push(5);
    println!("{:?}, len {}", v, v.len());
    println!("third = {}", v[2]);
    println!("safe get: {:?} {:?}", v.get(1), v.get(99)); // Option

    v.sort();
    v.dedup(); // remove consecutive duplicates
    println!("sorted+dedup: {:?}", v);

    for x in v.iter_mut() {
        *x *= 10; // modify in place through &mut
    }
    println!("{:?}", v);

    // ---------- String vs &str ----------
    // String: owned, growable.  &str: borrowed view of text.
    let mut s = String::from("Hello");
    s.push_str(", Rust");
    s.push('!');
    println!("{s} (len {})", s.len());
    println!("upper: {}", s.to_uppercase());
    println!("contains Rust? {}", s.contains("Rust"));
    println!("replace: {}", s.replace("Rust", "World"));

    let words: Vec<&str> = "the quick brown fox".split(' ').collect();
    println!("{:?}", words);
    let joined = words.join("-");
    println!("{joined}");

    // chars() iterates Unicode characters
    let reversed: String = "stressed".chars().rev().collect();
    println!("reversed: {reversed}");

    // ---------- HashMap<K, V> ----------
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Alice".to_string(), 90);
    scores.insert("Bob".to_string(), 75);
    println!("Alice: {:?}", scores.get("Alice"));

    // entry API: insert only if missing / update
    scores.entry("Carol".to_string()).or_insert(60);
    *scores.entry("Bob".to_string()).or_insert(0) += 5;

    let mut names: Vec<_> = scores.keys().collect();
    names.sort();
    for name in names {
        println!("{name} -> {}", scores[name]);
    }

    // Classic: word frequency
    let text = "one two three two three three";
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    let mut pairs: Vec<_> = freq.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("{:?}", pairs);

    // ---------- HashSet<T>: unique values ----------
    let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5].into_iter().collect();
    let mut common: Vec<_> = a.intersection(&b).collect();
    common.sort();
    println!("intersection: {:?}", common);
}

// TRY IT: Count how many times each character appears in a string.
// Then print the three most common ones.
