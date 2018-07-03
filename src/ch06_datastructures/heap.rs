use std::iter::FromIterator;
use std::slice::Iter;

pub struct Heap<T: PartialOrd> {
    elements: Vec<T>,
}

fn parent_index(index: usize) -> usize {
    (index - 1) / 2
}

impl<T: PartialOrd> Heap<T> {
    pub fn new() -> Self {
        Heap {
            elements: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
        let mut i = self.len() - 1;
        while i > 0 && self.elements[i] > self.elements[parent_index(i)] {
            self.elements.swap(i, parent_index(i));
            i = parent_index(i)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() > 0 {
            let result = self.elements.swap_remove(0);
            self.heapify(0);
            Some(result)
        } else {
            None
        }
    }

    fn heapify(&mut self, index: usize) {
        let mut largest = index;
        let left = 2 * index + 1;
        let right = 2 * index + 2;

        if left < self.len() && self.elements[left] > self.elements[largest] {
            largest = left
        }

        if right < self.len() && self.elements[right] > self.elements[largest] {
            largest = right
        }

        if largest != index {
            self.elements.swap(index, largest);
            self.heapify(index)
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.elements.iter()
    }
}

impl<T> FromIterator<T> for Heap<T>
where
    T: PartialOrd,
{
    fn from_iter<I: IntoIterator<Item = T>>(source: I) -> Self {
        let mut heap = Heap::new();

        for i in source {
            heap.push(i)
        }
        heap
    }
}

#[cfg(test)]
mod test {
    use super::Heap;

    #[test]
    fn test_new() {
        let heap: Heap<u32> = Heap::new();
        assert_eq!(heap.len(), 0);
    }

    #[test]
    fn test_push() {
        let mut heap = Heap::new();
        heap.push(5);
        assert_eq!(heap.len(), 1);
    }

    #[test]
    fn test_push_pop() {
        let mut heap = Heap::new();
        heap.push(5);
        assert_eq!(heap.pop().unwrap(), 5);
        assert_eq!(heap.len(), 0);
    }

    #[test]
    fn test_priority() {
        let mut heap = Heap::new();
        heap.push(5);
        heap.push(10);
        heap.push(7);
        heap.push(6);
        heap.push(3);
        heap.push(6);
        heap.push(7);
        heap.push(6);
        assert_eq!(heap.pop().unwrap(), 10);
        assert_eq!(heap.pop().unwrap(), 7);
        assert_eq!(heap.pop().unwrap(), 7);
        assert_eq!(heap.pop().unwrap(), 6);
        assert_eq!(heap.pop().unwrap(), 6);
        assert_eq!(heap.pop().unwrap(), 6);
        assert_eq!(heap.pop().unwrap(), 5);
        assert_eq!(heap.pop().unwrap(), 3);
        assert_eq!(heap.len(), 0);
    }

    #[test]
    fn test_from_iter() {
        let mut heap: Heap<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter().collect();
        assert_eq!(heap.len(), 10);
        assert_eq!(heap.pop().unwrap(), 10);
    }

    #[test]
    fn test_iter() {
        let heap: Heap<u32> = vec![1, 2, 3, 4, 5].into_iter().collect();
        let result: u32 = heap.iter().sum();
        assert_eq!(result, 15);
    }

}
