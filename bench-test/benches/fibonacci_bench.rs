// Note: this requires the nightly compiler
#![feature(test)]

extern crate test;
use test::Bencher;

/// bench-test fibonacci function
///
#[bench]
pub fn bench_fibonacci_10(b: &mut Bencher) {
    b.iter(|| {
        let _ = bench_test::fibonacci(10);
    });
}


#[bench]
pub fn bench_fibonacci_20(b: &mut Bencher) {
    b.iter(|| {
        let _ = bench_test::fibonacci(20);
    });
}

#[bench]
pub fn bench_fibonacci_50(b: &mut Bencher) {
    b.iter(|| {
        let _ = bench_test::fibonacci(30);
    });
}