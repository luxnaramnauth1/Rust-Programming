# Learn Rust: Beginner to Advanced

A hands-on course of 12 small, runnable programs. Each file teaches one topic, with comments and `println!` output so you can see what happens.

## 1. Setup

Install Rust (includes `rustc` and `cargo`):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
```

On Windows, download the installer from https://rustup.rs.

## 2. How to run a file

Quickest way (no project needed):

```bash
rustc 01_hello_variables.rs
./01_hello_variables        # Windows: 01_hello_variables.exe
```

Use edition 2021 for all files:

```bash
rustc --edition 2021 03_ownership_borrowing.rs
```

Or inside a Cargo project: copy a file to `src/main.rs`, then run `cargo run`.

Run the tests in file 12 with:

```bash
rustc --edition 2021 --test 12_final_project_inventory.rs && ./12_final_project_inventory
```

## 3. Learning path

| Level | File | Topics |
|-------|------|--------|
| Beginner | `01_hello_variables.rs` | `println!`, variables, mutability, shadowing, types, tuples, arrays |
| Beginner | `02_control_flow_functions.rs` | `if`, `loop`, `while`, `for`, ranges, functions, return values |
| Beginner | `03_ownership_borrowing.rs` | Ownership, move, clone, references, `&mut`, slices |
| Intermediate | `04_structs_enums_match.rs` | Structs, `impl`, enums, `match`, `Option`, `if let` |
| Intermediate | `05_collections_strings.rs` | `Vec`, `HashMap`, `String` vs `&str`, iteration |
| Intermediate | `06_error_handling.rs` | `Result`, `?`, custom errors, `From`, `Box<dyn Error>` |
| Intermediate | `07_traits_generics.rs` | Traits, generics, trait bounds, `impl Trait`, `dyn Trait` |
| Advanced | `08_lifetimes_iterators_closures.rs` | Lifetimes, closures, iterator adapters, custom iterators |
| Advanced | `09_smart_pointers.rs` | `Box`, `Rc`, `RefCell`, recursive types |
| Advanced | `10_concurrency.rs` | Threads, channels, `Arc<Mutex<T>>` |
| Advanced | `11_macros_unsafe.rs` | `macro_rules!`, `unsafe`, raw pointers |
| Project | `12_final_project_inventory.rs` | Everything combined, plus unit tests |

## 4. How to study

1. Read the file top to bottom, then run it.
2. Change values and break things on purpose. Compiler errors are Rust's best teacher.
3. Do the **Try it** exercise at the bottom of each file.
4. Move to the next file only when you can explain the current one.

## 5. Where to go next

- *The Rust Programming Language* ("the Book"): https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Rustlings (small exercises): https://github.com/rust-lang/rustlings
- Async Rust with `tokio`: https://tokio.rs
- Crates to learn: `serde`, `clap`, `reqwest`, `rayon`, `polars`
- Docs for any crate: https://docs.rs

## 6. Cheat sheet

```text
cargo new myapp      create a project
cargo run            build and run
cargo build --release optimized build
cargo test           run tests
cargo fmt            format code
cargo clippy         lint and suggestions
cargo add serde      add a dependency
```
