<!-- $theme: gaia -->
<!-- $size: 16:9 -->
<!-- page_number: true -->

# Rust
## A safe systems programming language![200% center](https://www.rust-lang.org/logos/rust-logo-blk.svg)

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
![center width: 150%](https://blog.rust-lang.org/images/2016-06-Survey/what_language.png)

---
# Hello World

```rust
fn main() {
	println!("hi!")
}
```

## Compile & Run

- not rusty (idiomatic), later more

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

---
## [clippy](https://github.com/Manishearth/rust-clippy)

- sophisticated linter
- `> 160` lints
- really nice for beginners to see antipatterns (index based loops instead of iterators, etc.)
- requires Rust nightly build (rustup :tada:) because the compiler plugin API is not stable, yet

---
# Pros

+ very friendly and helpful community - [code of conduct](https://www.rust-lang.org/conduct.html)
+ incredibly good [documentation](https://www.rust-lang.org/en-US/documentation.html)
+ great tooling
+ transparent language development, [RFC](https://github.com/rust-lang/rfcs)s, [internals](https://internals.rust-lang.org/).rust-lang.org

---
# Cons
- infrastructure to immature for productive use (depends)
- steep learning curve (fight against the borrow checker)
- debugger support could be better

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