#![allow(unused)]
#[allow(unused_imports)]

pub fn brute_force_search<T: PartialEq>(arr: &[T], el: T) -> bool {
    for i in 0..arr.len() {
        if arr[i] == el {
            return true;
        }
    }
    return false;
}

pub fn binary_search<T: Ord>(arr: &[T], el: T) -> bool {
    use std::cmp::Ordering;
    let mut remaining = arr;
    loop {
        let middle = remaining.len() / 2;
        match el.cmp(&remaining[middle]) {
            Ordering::Equal => return true,
            _ if remaining.len() == 1 => return false,
            Ordering::Less => remaining = &remaining[..middle],
            Ordering::Greater => remaining = &remaining[middle..],
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_brute_force_miss() {
        let vec: Vec<usize> = (0..100).collect();
        assert_eq!(brute_force_search(&vec, 101), false)
    }

    #[test]
    fn test_brute_force_hit() {
        let vec: Vec<usize> = (0..100).collect();
        assert_eq!(brute_force_search(&vec, 42), true)
    }

    #[test]
    fn test_binary_miss() {
        let vec: Vec<usize> = (0..100).collect();
        assert_eq!(binary_search(&vec, 101), false)
    }

    #[test]
    fn test_binary_hit() {
        let vec: Vec<usize> = (0..100).collect();
        assert_eq!(binary_search(&vec, 42), true)
    }

    #[bench]
    fn bench_brute_force(b: &mut test::Bencher) {
        let vec: Vec<usize> = (0..1000000).collect();
        b.iter(|| black_box(brute_force_search(&vec, 900000)));
    }

    #[bench]
    fn bench_binary_search(b: &mut test::Bencher) {
        let vec: Vec<usize> = (0..1000000).collect();
        b.iter(|| black_box(binary_search(&vec, 900000)));
    }

}
