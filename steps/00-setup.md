# 00 Setup

**Goal:** a Rust project in this folder that compiles and prints something.

## The problem

Rust is already installed on this machine (`cargo --version` reports 1.93). If you ever need it elsewhere, [rustup.rs](https://rustup.rs/) is the installer.

From inside this `qr-ascii` folder, run:

```
cargo init --name qr
```

That adds `Cargo.toml`, `src/main.rs` with a hello-world program, and a `.gitignore`. It also starts a git repository, which is handy: commit after every step, and you can always get back to something that worked.

Then get to know the four commands you'll use constantly:

| Command | What it does |
| --- | --- |
| `cargo run` | Build and run |
| `cargo check` | Type-check without building. Fast. Use it while you're fighting the compiler |
| `cargo test` | Run your tests. From step 09 on, this is how you check your work |
| `cargo clippy` | Lints. It suggests more idiomatic ways to write what you wrote |

`cargo fmt` formats your code, so you never have to think about layout.

In your editor, install **rust-analyzer**. Inline types and errors as you type make a big difference when you're learning.

## Rust you'll meet

- `fn main`, `println!`
- How a Cargo project is laid out

## Done when

- `cargo run` prints `Hello, world!`
- You've changed the message and run it again

## Read

- The Book, [chapter 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- The Book, [chapter 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html): variables, types, functions, control flow. Skim it now and come back as needed.
