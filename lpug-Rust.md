<!-- $theme: gaia -->
<!-- $size: 16:9 -->
<!-- page_number: true -->

# Rust
## A safe systems programming language![200% center](https://www.rust-lang.org/logos/rust-logo-blk.svg)

---
# Features

Some of Rust's selling points:

- **guaranteed memory safety** (current [research topic](http://plv.mpi-sws.org/rustbelt/))
- **threads without data races**
- generics
- pattern matching
- zero-cost abstractions ("What you don’t use, you don’t pay for")
- Rust balances control (unmanaged) and safety (memory managed languages

---
- compiled language :right_arrow: **no** GIL
	- the [rusti project](https://github.com/murarth/rusti) aims to provide a REPL for Rust
- no garbage collection :right_arrow: **no** GC pause
- Python is [*dynamic* and *strongly* typed](https://wiki.python.org/moin/Why%20is%20Python%20a%20dynamic%20language%20and%20also%20a%20strongly%20typed%20language)
- *static* strong typing with *type inference* in Rust
- Rust is a curly brace language, Python is indentation based

---
# Competition

Similar languages in order of common features:

- [nim](http://nim-lang.org/)
- C++
- [D](https://dlang.org/)
- [Swift](https://swift.org/)

---
# History

- in development since 2010
- first stable release (1.0) on May 15, 2015
- since then every three months a new minor versions, now 1.10
- open source project backed by Mozilla (and Samsung)

---
## Companies that use Rust
- [Servo](https://servo.org/), Mozilla's next generation parallel browser engine
- [Brotli decompressor](https://github.com/dropbox/rust-brotli) from Dropbox
- [... many more](https://www.rust-lang.org/en-US/friends.html)

---
# Trivia

- idiomatic Rust code is called  *rusty* or *rustic*
- Rust programmers are called *rustaceans* (like pythonistas)
- the language is named after a fungus
- [Ferris the Crab](http://www.rustacean.net/)
![25% center](http://www.rustacean.net/assets/rustacean-orig-noshadow.png)

---
# Python :couple: Rust
![center width: 150%](https://blog.rust-lang.org/images/2016-06-Survey/what_language.png)

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

# Compiled Language

- Python is interpreted (at runtime)
- Rust's compilation steps (MIR since 1.9):

Source :arrow_right: (HIR :arrow_right: [MIR](https://blog.rust-lang.org/2016/04/19/MIR.html) :arrow_right: LLVM IR :arrow_right:) Machine Code

- [LLVM](https://en.wikipedia.org/wiki/LLVM) intermediate representation (IR) is used by many compilers for a lot of different languages: clang (C), [Swift](https://swift.org/), [D](https://dlang.org/), [Haskell](https://www.haskell.org/), **[Pyston](https://github.com/dropbox/pyston)**, ...
- Rust benefits from improvements to LLVM's optimization steps

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
├── Cargo.toml
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

- immutable by deault
- create a variable binding: `let name: Type = value;`

---
# Ownsership

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
	1. one or more references &T to a resource (shared borrow)
	1. exactly one (even across threads) mutable reference &mut T (mutable borrow)

---
## Lifetimes

- references have lifetimes associated to it

---

# Shared mutable state is the root of all evil. :japanese_ogre:

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

- lazy evaluated
	- must be consumed to be evaluated: `collect()`
- can be infinite, e.g. `(0..3).cycle()`
- [cheat sheet](https://is.gd/bHG44p)

```rust
let v: Vec<f32> = vec![1.0, 33.0, -23.4];
let avg = v.iter().fold(0.0, |acc, val| (acc + val)/l);
```

---
# Traits [:pencil:](https://is.gd/VhL1GU)

- defines functionality an implementing type must provide
- composition over inheritance (avoids [diamond problem](https://en.wikipedia.org/wiki/Multiple_inheritance#The_diamond_problem))

```rust
use std::ops::Add;

struct Pair { a: f64, b: f64 }

impl Add for Pair {
    type Output = Pair;
    fn add(self, other: Pair) -> Pair {
        Pair { a: self.a + other.a,
            b: self.b + other.b, }
    }
}
```

---
# Generics [:pencil:](https://is.gd/bHG44p)

- parametrized types or functions
- one implementation for multiple types
- type parameters can be constrained, i.e. a number of traits can be specified that the input type must implement:

```rust
use std::io::Read;
fn reader<T: Read>(r: T) -> (usize, Vec<u8>) {
    let mut buf = Vec::new();
    let cnt = r.read(&mut buf).unwrap();
    (cnt, buf)
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
- hygienic macros, expand always to valid code at compile time (unlike C's text based preprocessor macros)

---
<!-- *template: invert -->
# DEMO: Call Rust from Python/Pypy

---
# Tools

## [rustfmt](https://github.com/rust-lang-nursery/rustfmt)

- `cargo install rustfmt`
- de-facto standard for code formatting, like go's `gofmt` but a bit less opionated
- `autopep8` ⩯ `rustfmt`

---
## [clippy](https://github.com/Manishearth/rust-clippy)

- sophisticated linter
- `> 160` lints
- really nice for beginners to see antipatterns (index based loops instead of iterators, etc.)
- requires Rust nightly build (rustup :tada:) because the compiler plugin API is not stable, yet

---
# Pros

+ very friendly and helpful community - [code of conduct](https://www.rust-lang.org/conduct.html)
+ powerful type system
+ safe concurrency
+ incredibly good [documentation](https://www.rust-lang.org/en-US/documentation.html)
+ great tooling
+ transparent language development, core team that decides about [RFC](https://github.com/rust-lang/rfcs)s, [internals](https://internals.rust-lang.org/).rust-lang.org
+ [play.rust-lang.org](http://play.rust-lang.org/)

---
# Cons
- infrastructure to immature for productive use (depends)
- steep learning curve (fight against the borrow checker)
- debugger support could be better
- immature ecosystem, mostly due to the short lifetime of Rust

---

# Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/) - The official Rust book
- [users.rust-lang.org](https://users.rust-lang.org/)
- [rustplatz](https://forum.rustplatz.de/)
- [/r/rust](https://www.reddit.com/r/rust/)
- [#rust-beginners](https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners) IRC
- [community overview](https://www.rust-lang.org/en-US/community.html)

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
