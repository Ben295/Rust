//! Game loop for the `hello_cargo` guessing game.
//!
//! All validation/parsing logic lives in `lib.rs` so it's testable and
//! benchmarkable. This binary is intentionally thin: read input, call
//! the library, print the result.

use hello_cargo::{parse_guess, GuessError};
use rand::Rng;
use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, Write};

const MIN: i32 = 1;
const MAX: i32 = 100;

fn main() {
    println!("Guess the number! (between {MIN} and {MAX})");

    // `rand::thread_rng()` gives a thread-local PRNG seeded from the OS.
    // `gen_range` is exclusive at the upper end with `..`, inclusive with `..=`.
    let secret: i32 = rand::thread_rng().gen_range(MIN..=MAX);
    let mut attempts: u32 = 0;

    // The borrow checker is visible here too:
    // - `input` is owned by the loop iteration. Each iteration creates a
    //   fresh `String` so there's no stale data from the last guess.
    // - `read_line` takes `&mut input` — an exclusive mutable borrow. While
    //   `read_line` runs, nothing else can read or write `input`. After
    //   it returns, the borrow ends and we own `input` freely again.
    loop {
        attempts += 1;
        print!("Attempt {attempts}: ");
        // `flush` makes the prompt appear before stdin blocks (otherwise
        // stdout is line-buffered and the prompt sits in the buffer).
        // `read_line` returns `io::Result<usize>`. `unwrap_or_else` makes
        // a stdout failure exit gracefully rather than panic.
        io::stdout()
            .flush()
            .unwrap_or_else(|e| eprintln!("(stdout flush failed: {e})"));

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("couldn't read input: {e}");
            return;
        }

        // The classic `match` on `Result`. `parse_guess` does both the
        // string→i32 parse AND range validation in one call, with `?`
        // chaining their errors into one `GuessError` enum.
        let guess = match parse_guess(&input, MIN, MAX) {
            Ok(g) => g,
            Err(e) => {
                // `Display` impl gives the human message; `source()` chains
                // down to the original ParseIntError if there is one.
                eprintln!("  error: {e}");
                if let Some(src) = e.source() {
                    eprintln!("  cause: {src}");
                }
                report_kind(&e);
                attempts -= 1; // invalid input doesn't count as an attempt
                continue;
            }
        };

        match guess.value().cmp(&secret) {
            Ordering::Less => println!("  too small"),
            Ordering::Greater => println!("  too big"),
            Ordering::Equal => {
                println!("  correct! {attempts} attempts.");
                break;
            }
        }
    }
}

/// Small helper showing pattern matching on the custom error enum.
///
/// In Java you'd do `if (e instanceof OutOfRange)`; Rust's `match` is
/// exhaustive — the compiler will tell you if you ever add a variant
/// to `GuessError` and forget to handle it here.
fn report_kind(err: &GuessError) {
    match err {
        GuessError::NotANumber(_) => eprintln!("  (kind: not a number)"),
        GuessError::OutOfRange { .. } => eprintln!("  (kind: out of range)"),
    }
}
