//! Criterion benchmarks for the parsing + validation hot path.
//!
//! Run with `cargo bench -p hello_cargo`. Criterion samples each
//! function many times to produce a statistical estimate of the mean
//! runtime along with confidence intervals — much more useful than
//! `start = Instant::now(); ...; start.elapsed()`, which only
//! captures one noisy sample.
//!
//! Why bench these tiny functions?
//! - The library is small enough that benches stay fast and the
//!   output is easy to read.
//! - It gives you a baseline to spot regressions when (later) you
//!   add features. If `parse_guess` jumps from ~50ns to ~5µs after
//!   a change, you want to know.
//!
//! Reduced sample size + measurement time below to keep the demo
//! fast. Production benches usually run with criterion's defaults
//! (100 samples, ~5s measurement time per function).

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hello_cargo::{parse_guess, Guess};

/// `black_box` tells the optimiser "pretend you don't know what's in
/// here." Without it, the compiler may notice the input is a constant
/// and pre-compute the answer at compile time, making the benchmark
/// measure nothing.
fn bench_parse_valid(c: &mut Criterion) {
    c.bench_function("parse_guess valid", |b| {
        b.iter(|| parse_guess(black_box("42"), 1, 100));
    });
}

fn bench_parse_invalid_text(c: &mut Criterion) {
    c.bench_function("parse_guess invalid text", |b| {
        b.iter(|| parse_guess(black_box("not-a-number"), 1, 100));
    });
}

fn bench_parse_out_of_range(c: &mut Criterion) {
    c.bench_function("parse_guess out of range", |b| {
        b.iter(|| parse_guess(black_box("999"), 1, 100));
    });
}

fn bench_new_in_range(c: &mut Criterion) {
    c.bench_function("Guess::new in-range i32", |b| {
        b.iter(|| Guess::new(black_box(42_i32), 1, 100));
    });
}

// `criterion_group!` collects benchmark functions. Custom config keeps
// the demo runtime modest; remove the `config = ...` line to use the
// criterion defaults.
criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(50)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2));
    targets = bench_parse_valid, bench_parse_invalid_text, bench_parse_out_of_range, bench_new_in_range
}

// `criterion_main!` generates the `main` for the benchmark binary.
// That's why `harness = false` was required in Cargo.toml — there's
// only one `main` per binary.
criterion_main!(benches);
