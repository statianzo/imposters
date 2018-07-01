use std::ptr;

type Link<T> = Option<Box<Node<T>>>;

pub struct Queue<T> {
    length: usize,
    front: Link<T>,
    rear: *mut Node<T>,
}

struct Node<T> {
    value: T,
    prev: Link<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            length: 0,
            front: Option::None,
            rear: ptr::null_mut(),
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn enqueue(&mut self, value: T) {
        let mut item = Box::new(Node {
            value,
            prev: Option::None,
        });

        let raw_rear: *mut _ = &mut *item;

        if self.rear.is_null() {
            self.front = Some(item);
        } else {
            unsafe {
                (*self.rear).prev = Some(item);
            }
        }

        self.rear = raw_rear;

        self.length = self.length + 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.front.take().map(|node| {
            let front = *node;
            self.front = front.prev;

            if self.front.is_none() {
                self.rear = ptr::null_mut();
            }

            front.value
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_queue() {
        let queue: Queue<i32> = Queue::new();
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_enqueue_len() {
        let mut queue = Queue::new();
        queue.enqueue(32);
        assert_eq!(queue.len(), 1);
        queue.enqueue(50);
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_deqeue() {
        let mut queue: Queue<u32> = Queue::new();
        let result = queue.dequeue();
        assert_eq!(result, Option::None);
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(32);
        queue.enqueue(50);
        assert_eq!(queue.dequeue().unwrap(), 32);
        assert_eq!(queue.dequeue().unwrap(), 50);
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_enq_after_deq() {
        // Specificially calls out hanging rear case
        let mut queue = Queue::new();
        queue.enqueue(32);
        queue.dequeue();
        queue.enqueue(100);
        assert_eq!(queue.dequeue().unwrap(), 100);
    }
}
