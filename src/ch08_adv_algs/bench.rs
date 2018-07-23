#![allow(unused)]
#![allow(dead_code)]

extern crate rand;
use super::fib;

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_fibonacci_nth(b: &mut Bencher) {
    b.iter(|| {
        let mut fib = fib::Fibonacci::new();
        black_box(fib.nth(150));
    });
}
