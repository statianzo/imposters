use std::iter::FromIterator;
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList { head: Option::None }
    }

    fn append(self, el: T) -> LinkedList<T> {
        LinkedList {
            head: Option::Some(Rc::new(Node {
                value: el,
                next: self.head,
            })),
        }
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn tail(&self) -> LinkedList<T> {
        LinkedList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
}

impl<'a, T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(source: I) -> Self {
        source
            .into_iter()
            .fold(LinkedList::new(), LinkedList::append)
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;
    use std::iter::FromIterator;

    #[test]
    fn test_append() {
        let list = LinkedList::<u32> { head: Option::None };
        let appended = list.append(42);
        match appended.head {
            Option::Some(node) => assert_eq!((*node).value, 42),
            _ => assert!(false, "was not Some"),
        }
    }

    #[test]
    fn test_append_str() {
        let list = LinkedList::<&str> { head: Option::None };
        let appended = list.append("one").append("two").append("three");
        match appended.head {
            Option::Some(node) => assert_eq!((*node).value, "three"),
            _ => assert!(false, "was not Some"),
        }
    }

    #[test]
    fn test_head() {
        let list = LinkedList::<&str> { head: Option::None };
        let appended = list.append("one").append("two").append("three");
        match appended.head() {
            Option::Some(val) => assert_eq!(val, &"three"),
            _ => assert!(false, "was not Some"),
        }
    }

    #[test]
    fn test_tail() {
        let list = LinkedList::<&str> { head: Option::None };
        let appended = list.append("one").append("two").append("three");
        match appended.tail().head() {
            Option::Some(val) => assert_eq!(val, &"two"),
            _ => assert!(false, "was not Some"),
        }
    }

    #[test]
    fn test_from_iter() {
        let list = LinkedList::from_iter(vec![1, 2, 3]);
        let h1 = list.head().unwrap().clone();
        let h2 = list.tail().head().unwrap().clone();
        let h3 = list.tail().tail().head().unwrap().clone();

        assert_eq!(h1, 3);
        assert_eq!(h2, 2);
        assert_eq!(h3, 1);
    }
}
