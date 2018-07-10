pub fn bubble_sort<T>(source: &mut [T])
where
    T: Ord,
{
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
}
