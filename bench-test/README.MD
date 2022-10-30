# BENCH TEST

run `cargo bench`

Note, this requires nightly version of rust. 
run ` rustup override set nightly` under the submodule to switch to nightly.

As of today (2022-10-30), bench is still unstable. If you want to use stable benchmark, you can use 3-rd party crates like criterion or benchmark.

See https://internals.rust-lang.org/t/pre-rfc-stabilize-bench-bencher-and-black-box/4565. 

## Benchmarks
```rust
#![feature(test)]

extern crate test;
use test::Bencher;
```

These 3 lines are required for bench tests. See [benchmark](https://course.rs/test/benchmark.html).

