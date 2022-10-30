# BENCH TEST

run `cargo bench`

Note, this requires nightly version of rust. 
run ` rustup override set nightly` under the submodule to switch to nightly.

## Benchmarks
```rust
#![feature(test)]

extern crate test;
use test::Bencher;
```

These 3 lines are required for bench tests. See [benchmark](https://course.rs/test/benchmark.html).
