use std::iter::FromIterator;
use std::mem;

#[derive(Debug, PartialEq)]
pub enum Node<T> {
    Cons(T, Box<Node<T>>),
    Nil,
}

impl<T> Node<T> {
    pub fn iter(self) -> Iter<T> {
        Iter { node: self }
    }
}

impl<'a, T> FromIterator<T> for Node<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut output = Node::Nil;

        for x in iter {
            output = Node::Cons(x, Box::new(output));
        }

        output
    }
}

pub struct Iter<T> {
    node: Node<T>,
}

impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let old_node = mem::replace(&mut self.node, Node::Nil);

        match old_node {
            Node::Cons(head, tail) => {
                self.node = *tail;
                Some(head)
            }
            Node::Nil => None,
        }
    }
}

#[cfg(test)]
mod test {
    use self::Node::*;
    use super::*;

    #[test]
    fn test_can_construct() {
        if let Cons(head, tail) = Cons("home", Box::new(Nil)) {
            assert_eq!(head, "home");
            assert_eq!(tail, Box::new(Nil));
        } else {
            assert!(false, "Not a Cons")
        }
    }

    #[test]
    fn test_from_iter() {
        let expected = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        let result = Node::from_iter(vec![3, 2, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        let i = list.iter();
        let result: Vec<i32> = i.collect();
        let expected = vec![1, 2, 3];

        assert_eq!(result, expected);
    }
}
