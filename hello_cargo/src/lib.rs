//! Library backing the `hello_cargo` guessing game.
//!
//! Pulled out of `main.rs` so the game logic is testable and
//! benchmarkable independently of stdio. Demonstrates several
//! Rust idioms a Java developer should drill on:
//!
//! - `Result<T, E>` for fallible operations and `?` for propagation
//! - A custom error type with `Display`, `Error::source`, and `From` impls
//! - Generics with trait bounds (`Guess<T>` works for any ordered type)
//! - Borrowing — public API returns `&T` from `value()` rather than
//!   moving `T` out, so the `Guess` keeps owning its data
//!
//! See `tests` at the bottom for usage examples.

use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;

// ──────────────────────────────────────────────────────────────────────────
// Generics
// ──────────────────────────────────────────────────────────────────────────

/// A value validated to lie within an inclusive `[min, max]` range.
///
/// Generic over `T` with three bounds:
/// - `Ord` so we can compare against bounds with `<` and `>`
/// - `Copy` so we can store/return the value cheaply (no clones)
/// - `Display` so error messages can format the value
///
/// Works for `i32`, `u8`, `u32`, `i64`, ... any built-in integer. In Java
/// you'd reach for `Comparable<T>`; here `Ord` plays the same role but is
/// resolved at compile time (monomorphisation) — `Guess<i32>` and
/// `Guess<u8>` are entirely separate types in the compiled binary, with
/// no virtual-call overhead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guess<T: Ord + Copy + Display> {
    value: T,
}

impl<T: Ord + Copy + Display> Guess<T> {
    /// Construct a `Guess`, returning `Err` if `value` is outside `[min, max]`.
    ///
    /// Returns a `Result`, the idiomatic Rust way to surface "this might
    /// fail" in the type system. Callers MUST handle the error variant —
    /// they can't accidentally ignore it (and the `unused_must_use`
    /// workspace lint enforces this).
    pub fn new(value: T, min: T, max: T) -> Result<Self, GuessError> {
        if value < min || value > max {
            return Err(GuessError::OutOfRange {
                value: value.to_string(),
                min: min.to_string(),
                max: max.to_string(),
            });
        }
        Ok(Self { value })
    }

    // ──────────────────────────────────────────────────────────────────
    // Borrow checker
    // ──────────────────────────────────────────────────────────────────

    /// Borrow the inner value.
    ///
    /// We return `&T`, not `T`, even though `T: Copy` would let us return
    /// by value. Why? Because returning `&self.value` is the lowest-cost
    /// option (no copy at all) and it teaches the right reflex: prefer
    /// borrowing over moving when the caller only needs to read.
    ///
    /// The `&self` parameter takes an *immutable* borrow of the Guess.
    /// While that borrow exists, no one — not even the Guess's owner —
    /// can mutate it. That's the borrow checker's two-rule contract:
    ///
    /// 1. Any number of `&T` (shared, read-only) refs OR exactly one `&mut T`.
    /// 2. References must always be valid (the compiler proves they
    ///    don't outlive their referent).
    ///
    /// For example, this would NOT compile:
    /// ```compile_fail
    /// # use hello_cargo::Guess;
    /// let mut g = Guess::new(5, 1, 10).unwrap();
    /// let a = g.value();           // immutable borrow of g
    /// let b = &mut g;              // ERROR: cannot also take &mut
    /// println!("{}", a);
    /// ```
    pub fn value(&self) -> &T {
        &self.value
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Result + ? operator
// ──────────────────────────────────────────────────────────────────────────

/// Parse `input` as an `i32` and wrap it in a `Guess` validated to lie in
/// `[min, max]`.
///
/// Two fallible operations in three lines:
/// 1. `.parse()` returns `Result<i32, ParseIntError>` — might fail if the
///    text isn't a number.
/// 2. `Guess::new()` returns `Result<_, GuessError>` — might fail if the
///    number is out of range.
///
/// The `?` operator does both jobs:
/// - On `Err`, unwrap and `return` it from the enclosing function.
/// - On `Ok`, unwrap and continue.
///
/// It also calls `From::from` to convert the inner error type to the
/// outer one — that's how a `ParseIntError` becomes a `GuessError`
/// automatically (see the `impl From<ParseIntError> for GuessError`
/// at the bottom of this file).
pub fn parse_guess(input: &str, min: i32, max: i32) -> Result<Guess<i32>, GuessError> {
    let value: i32 = input.trim().parse()?; // ?:  ParseIntError → GuessError via From
    Guess::new(value, min, max)
}

// ──────────────────────────────────────────────────────────────────────────
// Custom error type
// ──────────────────────────────────────────────────────────────────────────

/// Errors a guess can produce.
///
/// In Java you'd extend `Exception`; in Rust you implement traits.
/// The three impls below give this type:
/// - `Display`  — user-facing string formatting
/// - `Error`    — slots into Rust's error ecosystem (`Box<dyn Error>`, anyhow, ...)
/// - `From<E>`  — lets `?` automatically convert from another error type
#[derive(Debug)]
pub enum GuessError {
    /// Input wasn't parseable as a number.
    NotANumber(ParseIntError),
    /// Number was outside the allowed range.
    ///
    /// Bounds are stringified at construction so this variant doesn't
    /// need to be generic — keeps the error API simple while still
    /// reporting the actual values.
    OutOfRange {
        value: String,
        min: String,
        max: String,
    },
}

impl Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GuessError::NotANumber(_) => write!(f, "input wasn't a valid number"),
            GuessError::OutOfRange { value, min, max } => {
                write!(
                    f,
                    "{value} is out of range; expected between {min} and {max}"
                )
            }
        }
    }
}

impl Error for GuessError {
    /// Expose the underlying parse error as the source so callers can
    /// drill in. This is how error chains work in Rust: each layer
    /// returns its own message via `Display`, but the underlying cause
    /// is reachable via `source()`.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GuessError::NotANumber(e) => Some(e),
            GuessError::OutOfRange { .. } => None,
        }
    }
}

/// Enable `?` to convert `ParseIntError` to `GuessError` automatically.
impl From<ParseIntError> for GuessError {
    fn from(err: ParseIntError) -> Self {
        GuessError::NotANumber(err)
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_i32_is_ok() {
        let g = Guess::new(42_i32, 1, 100).expect("42 is in range");
        assert_eq!(*g.value(), 42);
    }

    #[test]
    fn below_min_is_err() {
        let err = Guess::new(0_i32, 1, 100).unwrap_err();
        assert!(matches!(err, GuessError::OutOfRange { .. }));
    }

    #[test]
    fn above_max_is_err() {
        let err = Guess::new(101_i32, 1, 100).unwrap_err();
        assert!(matches!(err, GuessError::OutOfRange { .. }));
    }

    #[test]
    fn boundaries_are_inclusive() {
        assert!(Guess::new(1_i32, 1, 100).is_ok());
        assert!(Guess::new(100_i32, 1, 100).is_ok());
    }

    #[test]
    fn parse_accepts_whitespace() {
        let g = parse_guess("  17\n", 1, 100).expect("17 is in range");
        assert_eq!(*g.value(), 17);
    }

    #[test]
    fn parse_rejects_garbage() {
        let err = parse_guess("nope", 1, 100).unwrap_err();
        assert!(matches!(err, GuessError::NotANumber(_)));
        // Error chain is reachable via source().
        assert!(err.source().is_some());
    }

    #[test]
    fn parse_propagates_out_of_range() {
        let err = parse_guess("999", 1, 100).unwrap_err();
        assert!(matches!(err, GuessError::OutOfRange { .. }));
    }

    #[test]
    fn generic_struct_works_with_u8() {
        // Same `Guess` type, different `T`. The compiler generates a
        // separate `Guess<u8>` from `Guess<i32>` (monomorphisation).
        let g = Guess::new(200_u8, 0, 255).expect("200 is in range");
        assert_eq!(*g.value(), 200);
    }

    #[test]
    fn display_formats_out_of_range_message() {
        let err = Guess::new(7_i32, 10, 20).unwrap_err();
        assert_eq!(
            err.to_string(),
            "7 is out of range; expected between 10 and 20"
        );
    }
}
