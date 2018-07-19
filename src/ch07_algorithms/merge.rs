use std::cmp::Ordering;

fn merge<T>(source: &mut [T])
where
    T: Ord + Clone,
{
    let middle = source.len() / 2;
    let mut left_range = 0..middle;
    let mut right_range = middle..source.len();
    let mut ordered: Vec<T> = Vec::with_capacity(source.len());

    while left_range.len() > 0 && right_range.len() > 0 {
        match source[left_range.start].cmp(&source[right_range.start]) {
            Ordering::Greater => {
                ordered.push(source[right_range.start].clone());
                right_range.next();
            }
            _ => {
                ordered.push(source[left_range.start].clone());
                left_range.next();
            }
        }
    }

    for index in left_range.chain(right_range) {
        ordered.push(source[index].clone())
    }

    source.swap_with_slice(&mut ordered);
}

pub fn merge_sort<T>(source: &mut [T])
where
    T: Ord + Clone,
{
    if source.len() <= 1 {
        return;
    }

    let middle = source.len() / 2;
    merge_sort(&mut source[0..middle]);
    merge_sort(&mut source[middle..]);
    merge(source)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small_sort() {
        let mut my_vec = vec![100, 42, 73, 24];
        merge_sort(&mut my_vec);
        assert_eq!(my_vec, vec![24, 42, 73, 100])
    }

    quickcheck! {
        fn test_any_sort(xs : Vec<u32>) -> bool {
            let mut expected = xs.clone();
            expected.sort();
            let mut xs = xs;
            merge_sort(&mut xs);
            xs == expected
        }
    }
}
