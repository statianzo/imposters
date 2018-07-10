pub fn insertion_sort<T>(source: &mut [T])
where
    T: Ord,
{
    for i in 0..source.len() {
        for j in (0..i).rev() {
            if source[j] > source[j + 1] {
                source.swap(j, j + 1);
            } else {
                break;
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
        insertion_sort(&mut my_vec);
        assert_eq!(my_vec, vec![24, 42, 73, 100])
    }

    quickcheck! {
        fn test_any_sort(xs : Vec<u32>) -> bool {
            let mut expected = xs.clone();
            expected.sort();
            let mut xs = xs;
            insertion_sort(&mut xs);
            xs == expected
        }
    }
}
