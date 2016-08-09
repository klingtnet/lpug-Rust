<!-- $theme: gaia -->
<!-- $size: 16:9 -->

# Rust
## A safe systems programming language![200% center](https://www.rust-lang.org/logos/rust-logo-blk.svg)

---
<!-- page_number: true -->
# Features

Some of Rust's selling points:

- **guaranteed memory safety** (current [research topic](http://plv.mpi-sws.org/rustbelt/))
- **threads without data races**
- generics
- pattern matching
- zero-cost abstractions ("What you don’t use, you don’t pay for.")
- Rust balances control (unmanaged) and safety (memory managed languages

---
# Quick Comparison

|Rust|Python|
|:-:|:-:|
|compiled|interpreted (GIL)|
|*static* strong typing with *type inference*|[*dynamic* and *strongly* typed](https://wiki.python.org/moin/Why%20is%20Python%20a%20dynamic%20language%20and%20also%20a%20strongly%20typed%20language)|
|automatic¹ /wo GC ➡ real time capable |garbage collected|
|`{` curly brace language `}`|indentation based|

¹ you have to think about variable scopes 

---
# History

- in development since 2010
- first stable release (1.0) on May 15, 2015
- since then about every 6 weeks a new minor versions
	- 1.10 at the time of writing
- open source project backed by Mozilla, Samsung and others

---
# Competition

Similar languages in order of common features:

- [nim](http://nim-lang.org/)
- C++
- [D](https://dlang.org/)
- [Swift](https://swift.org/)

---
## Companies that use Rust
- [Servo](https://servo.org/), Mozilla's next generation parallel browser engine
- [Brotli decompressor](https://github.com/dropbox/rust-brotli) from Dropbox
- [... many more](https://www.rust-lang.org/en-US/friends.html)

Rust is often used as a replacement for C to write safe and performant external modules for Ruby (RoR) or Python applications.

---
# Trivia

- Rust programmers are called *rustaceans* (like pythonistas)
- the language is named after a fungus
- idiomatic Rust code is called  *rusty* or *rustic*
- [Ferris the Crab](http://www.rustacean.net/)
![25% center](http://www.rustacean.net/assets/rustacean-orig-noshadow.png)

---
# Rust :couple: Python
![center width: 150%](https://blog.rust-lang.org/images/2016-06-Survey/what_language.png)

---
# Compiled Language
	
- Python is interpreted (at runtime)
- Rust's compilation steps (MIR since 1.9):

Source ➡ (HIR ➡ [MIR](https://blog.rust-lang.org/2016/04/19/MIR.html) ➡ LLVM IR ➡) Machine Code

- [C]Python: source (.py) ➡ bytecode (.pyc) ➡ runtime
- [LLVM](https://en.wikipedia.org/wiki/LLVM) intermediate representation (IR) is used by many compilers for a lot of different languages: clang (C), [Swift](https://swift.org/), [D](https://dlang.org/), [Haskell](https://www.haskell.org/), **[Pyston](https://github.com/dropbox/pyston)**, ...
- Rust benefits from improvements of LLVM's optimization steps

---
# Hello World

```rust
fn main() {
   let words = ["Hello", "pythonistas"];
   for word in words.iter() {
       print!("{} ", word);
   }
   println!("!");
}
```

## Compile & Run

```sh
$ rustc hello.rs -o hello && ./hello
Hello pythonistas !
```
---

# Installation

- Linux, Win (e.g. cygwin bash), OSX:

```sh
curl https://sh.rustup.rs -sSf | sh
```

- Rust comes in different flavours: *stable, beta, nightly*
- [`rustup`](https://rustup.rs) (former multirust) lets you install different Rust versions side-by-side
- `rustup` also manages target platforms for cross-compilation 
- Rust's package manager called [`cargo`](https://crates.io) is also part of the bundle

---

# Cargo

- Rust's package manager and build tool
- Packages are called *crates*
- ~5.6k crates on [crates.io](https://crates.io/)
- **No** batteries included but there is a crate for everything
- test runner, dependency managament (pip), documentation generator, ...
- additional features through plugins

---

```sh
$ cargo --list
Installed Commands:
    bench
    build
    check
    clean
    clippy
    doc
    fetch
    fmt
    generate-lockfile
    git-checkout
    graph
    help
    init
    install
    locate-project
    login
    ...
```
---

Setup a new project (`--bin` application, default is library):

```sh
$ cargo new --bin example
$ tree example
example
├── Cargo.toml // ~requirements.txt && setup.py
└── src/
    └── main.rs
```
```sh
$ cargo run
   Compiling example v0.1.0 (file:///home/andreas/personal/presentations/lpug-Rust/example)
     Running `target/debug/example`
Hello, world!
```
---
# Variables

- immutable by default
- create a variable binding: `let name: type = value;`
- mutable binding: `let mut x: type = value;`
- must be initialized with some value
- `type` can often be inferred

---
# Ownership

- variable *bindings have ownership*
- if a binding goes out of scope its resources are freed
- Rust ensures that there is only **one** binding to any resource

---
## Borrowing

- *borrowing* is lending a reference `&T` to a resource

```rust
sum(&[1,2,3,4,5]);
```

- resource is not freed when reference gets out of scope
- **borrowing rules**:
	1. one or more references `&T` to a resource (shared borrow)
	1. exactly one (even across threads) mutable reference `&mut T` (mutable borrow)

---
## Lifetimes

- references have lifetimes
- a reference can't outlive the resource it is pointing at
- if the compiler can't infer the lifetime then it must be declared

```rust
// implicit
fn foo(x: &i32) {
}

// explicit
fn bar<'a>(x: &'a i32) {
}
```

---
# Enums

- Rust's enums are *algebraic datatypes*
	- **algebraic**: build from product types (tuples, structs) and sum types (only one variant at any one time, e.g. enum variants)

```rust
enum Message {
    Quit, // variant /wo data
    ChangeColor(i32, i32, i32), // tuple variant
    Move { x: i32, y: i32 }, // struct (≈dict) variant
    Write(String), // single value variant
}
```
---
# Pattern Matching [:pencil:](https://is.gd/pXyveU)

```rust
enum E {
    A,
    B(i32)
}

fn main() {
    let e = E::B(4);
    match e {
        E::A => println!("I'm an A!"),
        E::B(x) => println!("I'm an B with value: {}!",x),
    }
}
```

---
# Iterators [:pencil:](http://is.gd/U3mXdV)

- list/dict comprehensions on stereoids
- lazy evaluated, must be consumed to get the value
- get a vector with values from 0 to 9: `let v = (0..10).collect::<Vec<_>>::();`
- [iterator cheat sheet](https://danielkeep.github.io/itercheat_baked.html)

---
# Traits [:pencil:](https://is.gd/VhL1GU)

- defines functionality an implementing type must provide

```rust
use std::ops::Add;

struct Pair { a: f64, b: f64 }

impl Add for Pair {
    type Output = Pair;

    fn add(self, other: Pair) -> Pair {
        Pair {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}
```

---
# Generics [:pencil:](http://is.gd/StM1Wg)

- parameterized type

```rust
use std::ops::Add;

// looks a bit strange, but
// the output can be different from the input
// type.
fn add<T: Add<T>>(x: T, y: T) -> T::Output {
    x + y
}

fn main() {
    println!("{}", add(3.0, 0.14159265359));
    println!("{}", add(30_000u64, 40_000));
}
```

---
# Error Handling

- **no exceptions**, errors are values (like in Go)

```rust
fn read_file(path: Into<String>) -> Result<Vec<u8>,IOError> { ... }

let data = match read_file("./some.file")  {
    Ok(data) => data,
    Err(error) => panic!(err),
}
```

- there are certain macros and methods to avoid explicit matching of common `Option` and `Result` types ([`try!`](https://doc.rust-lang.org/1.10.0/std/macro.try!.html))

```
// shorter version from above
let data = read_file("./some.file").expect("could not read file");
```

---
# Macros

- end with an `!`: `println!`
- expandend at compile time
- hygienic macros: expand always to valid code at compile time (unlike C's text based preprocessor macros)

---
# Tools

## [rustfmt](https://github.com/rust-lang-nursery/rustfmt)

- `cargo install rustfmt`
- de-facto standard for code formatting, like go's `gofmt` but a bit less opionated
- `autopep8` ⩯ `rustfmt`
- the [rusti project](https://github.com/murarth/rusti) aims to provide a REPL for Rust

---
## [clippy](https://github.com/Manishearth/rust-clippy)

- sophisticated linter
- `> 160` lints
- really nice for beginners to see antipatterns (index based loops instead of iterators, etc.)
- requires Rust nightly build (`rustup` :tada:) because the compiler plugin API is not stable yet

---
# Pros

+ very friendly and helpful community - [code of conduct](https://www.rust-lang.org/conduct.html)
+ incredibly good [documentation](https://www.rust-lang.org/en-US/documentation.html)
+ great tooling
+ transparent language development, core team that decides about [RFC](https://github.com/rust-lang/rfcs)s, [internals.rust-lang.org](https://internals.rust-lang.org/)
+ [play.rust-lang.org](http://play.rust-lang.org/)

---
# Cons
- infrastructure too immature for productive use (depends)
- steep learning curve (fight against the borrow checker)
	- (+) learning Rust will give you a new perspective on how your code is executed
- debugger support could be better

---

# Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/) - The official Rust book
- [users.rust-lang.org](https://users.rust-lang.org/)
- [rustplatz](https://forum.rustplatz.de/)
- [/r/rust](https://www.reddit.com/r/rust/)
- [#rust-beginners](https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners) IRC
- [community overview](https://www.rust-lang.org/en-US/community.html) or [rustaceans.org](http://www.rustaceans.org/)

---
<!-- *template: gaia -->
# ![width: 300%](/home/andreas/personal/presentations/lpug-Rust/rustfest.svg)
## See you @ [Rustfest](https://avatars3.githubusercontent.com/u/18737980?v=3&s=200)?
### First european Rust conference
### September, 17th and 18th in Berlin

---
<!-- *template: invert -->
# Questions?

---
<!-- *template: gaia -->
# Thank you for listening!
## Made with :heart: and [marp editor](https://yhatt.github.io/marp/)
### klingt.net
### Andreas Linz - 2016
