# Rust

## A safe systems programming language

---

- idiomatic Rust: *rusty* or *rustic*
- Rust programmers are called *rustaceans* (like pythonistas)

---
# Hello World

```rust
fn main() {
	println!("hi!")
}
```

## Compile & Run

- not idiomatic, later more

```sh
$ rustc hello.rs -o hello && ./hello
hi!
```
---

# Installation

- Rust comes in different flavours: *stable, beta, nightly*
- [`rustup`](https://rustup.rs) (former multirust) lets you install different Rust versions side-by-side
	 - this is (probably) **not** possible using your OS's package manager
	 - `rustup` also provides [`cargo`](https://crates.io), Rust's package manager
	 - provides various targets for cross-compilation 
- Linux, Win (e.g. cygwin bash), OSX:

```sh
curl https://sh.rustup.rs -sSf | sh
```
---

# Compiled

Source `>>` HIR `>>` [MIR](https://blog.rust-lang.org/2016/04/19/MIR.html) `>>` LLVM IR `>>` Machine Code

- [LLVM](https://en.wikipedia.org/wiki/LLVM) intermediate representation (IR) is used by many compilers for a lot of different languages: clang (C), [Swift](https://swift.org/), [D](https://dlang.org/), [Haskell](https://www.haskell.org/), **[Pyston](https://github.com/dropbox/pyston)** ...
- From improvements in LLVM's IR optimization steps benefit all languages (that are using LLVM)

---

# Cargo

```sh
$ cargo new --bin example
$ tree example
example
├── Cargo.toml
└── src
    └── main.rs
```
```sh
$ cargo run
   Compiling example v0.1.0 (file:///home/andreas/personal/presentations/lpug-Rust/example)
     Running `target/debug/example`
Hello, world!
```
---
# Enums

- Rust's enums are *algebraic datatypes*
	- algebraic because ... `+`,`*`

---
# Iterators

---
# Traits

---
# Generics

---
# Macros

---
# Batteries included?

---
# Tools

## [rustfmt](https://github.com/rust-lang-nursery/rustfmt)

- `cargo install rustfmt`
- de-facto standard for code formatting, like go's `gofmt` but a bit less opionated
- `autopep8` ⩯ `rustfmt`

## [clippy](https://github.com/Manishearth/rust-clippy)

- sophisticated linter
- `> 160` lints
- really nice for beginners to see antipatterns (for loops)

---
# Questions?

---

# klingt.net
## Andreas Linz - 2016