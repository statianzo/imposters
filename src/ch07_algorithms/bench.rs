#![allow(unused)]
#![allow(dead_code)]

extern crate rand;
use super::bubble;
use super::heap;
use super::insertion;
use super::merge;
use super::selection;
use ch07_algorithms::bench::rand::Rng;

extern crate test;
use test::{black_box, Bencher};

fn generate_data() -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut v = vec![0u32; 1000];
    for x in v.iter_mut() {
        *x = rng.gen();
    }
    v
}

#[bench]
fn bench_selection_sort(b: &mut Bencher) {
    let data = generate_data();
    b.iter(|| {
        let mut data_clone = data.clone();
        black_box(selection::selection_sort(&mut data_clone))
    });
}

#[bench]
fn bench_bubble_sort(b: &mut Bencher) {
    let data = generate_data();
    b.iter(|| {
        let mut data_clone = data.clone();
        black_box(bubble::bubble_sort(&mut data_clone))
    });
}

#[bench]
fn bench_insertion_sort(b: &mut Bencher) {
    let data = generate_data();
    b.iter(|| {
        let mut data_clone = data.clone();
        black_box(insertion::insertion_sort(&mut data_clone))
    });
}

#[bench]
fn bench_merge_sort(b: &mut Bencher) {
    let data = generate_data();
    b.iter(|| {
        let mut data_clone = data.clone();
        black_box(merge::merge_sort(&mut data_clone))
    });
}

#[bench]
fn bench_heap_sort(b: &mut Bencher) {
    let data = generate_data();
    b.iter(|| {
        let mut data_clone = data.clone();
        black_box(heap::heap_sort(&mut data_clone))
    });
}

#[bench]
fn bench_builtin_sort(b: &mut Bencher) {
    let data = generate_data();
    b.iter(|| {
        let mut data_clone = data.clone();
        black_box(data_clone.sort());
    });
}

#[bench]
fn bench_builtin_unstable_sort(b: &mut Bencher) {
    let data = generate_data();
    b.iter(|| {
        let mut data_clone = data.clone();
        black_box(data_clone.sort_unstable());
    });
}
