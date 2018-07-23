pub struct Fibonacci {
    current: u128,
    after: u128,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci {
            current: 0,
            after: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current = self.after;
        self.after = self.current + result;
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next() {
        let mut fib = Fibonacci::new();

        assert_eq!(fib.next(), Some(0));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), Some(8));
    }

    #[test]
    fn test_iterates() {
        let mut fib = Fibonacci::new();

        assert_eq!(fib.nth(150), Some(9969216677189303386214405760200));
    }
}
