# Learning Rust

## Installing Rust

1. Install rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

2. Update rust
rustup update

3. Check rust version
rustup --version

## Rust for those who know C++

https://www.youtube.com/watch?v=IPmRDS0OSxM

## Rust language book

https://doc.rust-lang.org/

### To classify

### Chapter 01 - Getting Started

2. `hello_cargo` `./projects/hello_cargo/`
1. `hello_world` `./projects/hello_world/`

### Chapter 02 - Programming a Guessing Game

1. `guessing_game` `./projects/guessing_game/`

### Chapter 03 - Common Programming Concepts

1. `variables` `./projects/variables/`
2. `data_types` `./projects/data_types/`
3. `functions` `./projects/functions/`
4. `control_flow` `./projects/control_flow/`

### Chapter 04 - Understanding Ownership

1. `ownership` `./projects/ownership/`
2. `references` `./projects/references/`
3. `slices` `./projects/slices/`

### Chapter 05 - Using Structs

1. structs ./projects/structs/

### Chapter 05 - Enums and Pattern Matching

1. enums ./projects/enums/
2. `match_operator` `./projects/match_operator/`

### Chapter 07 - Managing Growing Projects

1. restaurant ./projects/restaurant/

### Chapter 08 - Common Collections

1. `vector_type` `./projects/vector_type/`
2. `string_type` `./projects/string_type/`
3. `hashmap_type` `./projects/hashmap_type/`

### Chapter 09 - Error Handling

1. `error_handling` `./projects/error_handling/`

### Chapter 10 - Generic Types, Traits, and Lifetimes

1. `generic_types` `./projects/generic_types/`
2. traits ./projects/traits/
3. lifetimes ./projects/lifetimes/

### Chapter 12 - An I/O Project: Building a command line program

1. minigrep ./projects/minigrep/

### Chapter 13 - Functional Language Features: Iterators and Closures

1. closures ./projects/closures/
2. iterators ./projects/iterators/

### Chapter 14 - More about Cargo and Crates.io

### Chapter 15 - Smart Pointers

1. `box_ptr` `./projects/box_ptr/`
2. deref ./projects/deref/
3. drop ./projects/drop/
4. rc ./projects/rc/
5. refcell ./projects/refcell/
6. `ref_cycles` `./projects/ref_cycles/`

### Chapter 16 - Fearless Concurrency

1. threads ./projects/threads/
2. `message_passing` `./projects/message_passing/`
3. `shared_state` `./projects/shared_state/`
4. `sync_send` `./projects/sync_send`

### Chapter 17 - Object Oriented Programming Features of Rust

1. encapsulation `./projects/encapsulation_impl/`
2. polymorphism `./projects/polymorphism_traits/`
3. `ood_pattern` `./projects/ood_pattern`

### Chapter 18 - Patterns and Matching

1. `pattern_locations` `./projects/pattern_locations/`
2. `pattern_syntax` `./projects/pattern_syntax/`

## Rust language book remaining

### Chapter 11 - Writing Automated Tests

1. adder
2. `running_tests`

## Rustlings

Learn Rust with small exercises.

### Install

1. Lookup the latest version at https://github.com/rust-lang/rustlings/releases/latest

2. Clone the code

```
git clone -b 5.2.1 --depth 1 https://github.com/rust-lang/rustlings
```

3. Setup the project

```
cd rustlings
cargo install --force --path .
```

4. Move the rustlings directory to ~/

This makes WSL watch work and also runs the code faster.

## Rust concepts

https://rauljordan.com/rust-concepts-i-wish-i-learned-earlier/

## Links

* [Rust programming language][1000] book

[1000]: https://doc.rust-lang.org/stable/book/

* Rust [podcasts][1010]

[1010]: https://newrustacean.com/show_notes/index.html

* How to [learn Rust][1020]

[1020]: https://www.youtube.com/watch?v=sDtQaO5_SOw

* From [Python to Rust][1030] videos

[1030]: https://www.youtube.com/playlist?list=PLEIv4NBmh-GsWGE9mY3sF9c5lgh5Z_jLr

* The [Rust programming language][1040]

[1040]: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/

* Practical guide to [async in Rust][1050]

[1050]: https://blog.logrocket.com/a-practical-guide-to-async-in-rust/
