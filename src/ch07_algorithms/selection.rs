pub fn selection_sort<T>(source: &mut [T])
where
    T: Ord,
{
    for cur_index in 0..source.len() {
        let (min_index, _) = source
            .iter()
            .enumerate()
            .skip(cur_index)
            .min_by_key(|&(_, item)| item)
            .unwrap();
        source.swap(cur_index, min_index);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small_sort() {
        let mut my_vec = vec![100, 42, 73, 24];
        selection_sort(&mut my_vec);
        assert_eq!(my_vec, vec![24, 42, 73, 100])
    }

    quickcheck! {
        fn test_any_sort(xs : Vec<u32>) -> bool {
            let mut expected = xs.clone();
            expected.sort();
            let mut xs = xs;
            selection_sort(&mut xs);
            xs == expected
        }
    }
}
