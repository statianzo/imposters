fn build_heap<T>(source: &mut [T])
where
    T: Ord,
{
    for i in (0..source.len()).rev() {
        heapify(source, i)
    }
}

fn heapify<T>(source: &mut [T], index: usize)
where
    T: Ord,
{
    let mut largest = index;
    let left = 2 * index + 1;
    let right = left + 1;

    if left < source.len() && source[left] > source[largest] {
        largest = left
    }

    if right < source.len() && source[right] > source[largest] {
        largest = right
    }

    if largest == index {
        return;
    }

    source.swap(index, largest);

    if largest < source.len() {
        heapify(source, largest)
    }
}
pub fn heap_sort<T>(source: &mut [T])
where
    T: Ord,
{
    build_heap(source);

    for i in (0..source.len()).rev() {
        source.swap(0, i);
        heapify(&mut source[0..i], 0);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small_sort() {
        let mut my_vec = vec![100, 42, 73, 24, 300];
        heap_sort(&mut my_vec);
        assert_eq!(my_vec, vec![24, 42, 73, 100, 300])
    }

    #[test]
    fn test_dud() {
        let mut my_vec = vec![0, 0, 0, 1, 2];
        heap_sort(&mut my_vec);
        assert_eq!(my_vec, vec![0, 0, 0, 1, 2])
    }

    quickcheck! {
        fn test_any_sort(xs : Vec<u32>) -> bool {
            let mut expected = xs.clone();
            expected.sort();
            let mut xs = xs;
            heap_sort(&mut xs);
            xs == expected
        }
    }

}
