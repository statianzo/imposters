pub fn bubble_sort<T>(source: &mut [T])
where
    T: Ord,
{
    if source.len() == 0 {
        return;
    }
    let mut keep_going = true;
    while keep_going {
        keep_going = false;
        for i in 0..source.len() - 1 {
            if source[i] > source[i + 1] {
                source.swap(i, i + 1);
                keep_going = true;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small_sort() {
        let mut my_vec = vec![100, 42, 73, 24];
        bubble_sort(&mut my_vec);
        assert_eq!(my_vec, vec![24, 42, 73, 100])
    }

    quickcheck! {
        fn test_any_sort(xs : Vec<u32>) -> bool {
            let mut expected = xs.clone();
            expected.sort();
            let mut xs = xs;
            bubble_sort(&mut xs);
            xs == expected
        }
    }
}
