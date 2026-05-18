# Rust

A personal workspace for learning Rust, framed for a Java developer preparing for a Rust interview.

## Why this repo exists

A sandbox for hands-on Rust practice — small, focused exercises that exercise core concepts, rather than one big project. The aim is to get fluent in the parts of Rust that don't map cleanly from Java, fast enough to be useful in an interview.

The things that need real practice (vs. just reading about them):

- **Ownership, borrowing, and lifetimes** — Rust's defining feature; no Java equivalent
- **`Option<T>` and `Result<T, E>`** — Rust's answer to `null` and checked exceptions
- **`enum` and pattern matching** — far richer than Java's `enum`; variants carry data
- **Traits and generics** — Rust's take on interfaces and parametric types
- **Modules and crates** — how Rust projects are organised (`cargo`, `Cargo.toml`)
- **`async`, `Send`, `Sync`** — concurrency without a GC

## Getting started

### 1. Install Rust

Rust is installed via `rustup`, the official toolchain manager:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart your shell, then verify:

```bash
rustc --version
cargo --version
```

`cargo` is Rust's build tool *and* package manager — Maven/Gradle plus rustc plus a test runner, all in one binary that ships with the language.

### 2. VS Code setup

Install [VS Code](https://code.visualstudio.com/), then add these extensions:

- **rust-analyzer** (`rust-lang.rust-analyzer`) — the LSP. Inline types, hover docs, code actions. Essential.
- **CodeLLDB** (`vadimcn.vscode-lldb`) — debugger.
- **Even Better TOML** (`tamasfe.even-better-toml`) — `Cargo.toml` syntax + completions.

Open the workspace:

```bash
code ~/Documents/Claude/Projects/Rust
```

### 3. Create an exercise

Each exercise is its own Cargo project under this repo:

```bash
cargo new hello_world
cd hello_world
cargo run
```

`cargo new` creates `src/main.rs`, `Cargo.toml`, and a local `.gitignore`.

## Workflow

```bash
cargo check         # type-check without producing a binary (fastest feedback)
cargo build         # compile
cargo run           # compile + run
cargo test          # run all #[test] functions (and doc tests)
cargo clippy        # lints — run this often, it's an excellent teacher
cargo fmt           # auto-format
```

Commit cycle:

```bash
cargo test && cargo clippy
git add .
git commit -m "Add <exercise name>"
git push
```

## Java → Rust mental model

| Java                                | Rust                                                                 |
|-------------------------------------|----------------------------------------------------------------------|
| `class Foo { ... }`                 | `struct Foo { ... }` with separate `impl Foo { ... }` blocks         |
| `interface Foo`                     | `trait Foo`                                                          |
| `null`                              | `Option<T>` — `Some(x)` or `None`                                    |
| Checked exception                   | `Result<T, E>` — `Ok(x)` or `Err(e)`                                 |
| `try { } catch { }`                 | `?` operator propagates `Result`/`Option` errors up the call stack   |
| `new Foo()` (GC'd heap)             | `Box::new(Foo {})` — single owner on the heap, dropped on scope exit |
| `final` field / local               | `let` is final by default; `let mut` to allow reassignment           |
| `synchronized` block                | `Mutex<T>` / `RwLock<T>`, usually wrapped in `Arc<T>` to share       |
| `enum Color { RED, GREEN }`         | `enum Color { Red, Green }` — variants can also carry data           |
| Maven / Gradle                      | `cargo` + `Cargo.toml`                                               |
| `package com.foo.bar`               | `mod` within a crate; separate crates across project boundaries      |
| `T extends Comparable<T>`           | `T: Ord` (trait bound)                                               |
| Generic erasure                     | Monomorphisation — each concrete type gets its own compiled copy     |

The biggest mental shift: **no garbage collector**. The compiler statically tracks who owns each value and when it's dropped. Most "fighting the borrow checker" early on is trying to write Java-shaped code in Rust — e.g. holding multiple mutable references to the same data, or returning a reference to a local.

## Interview focus topics

Beyond the basics:

- The three ownership rules and the two borrow rules
- `&T` vs `&mut T` — and why the compiler allows many of one or one of the other, but not both
- Lifetimes: when you need annotations vs. when elision handles it
- `Copy` vs `Clone` vs `Drop` — and why `String` isn't `Copy` but `i32` is
- `Send` and `Sync` — what makes a type safe to move between or share across threads
- Zero-cost abstractions — iterators (lazy, fused at compile time), `Box<dyn Trait>` (dynamic dispatch) vs generics (static dispatch)
- `async`/`await` and the `Future` trait at a high level — Rust's `Future` is a state machine the executor polls

## Repo layout

```
.
├── AGENTS.md         # Guidance for AI coding agents working in this repo
├── README.md         # This file
└── <exercise>/       # One Cargo project per exercise
```
