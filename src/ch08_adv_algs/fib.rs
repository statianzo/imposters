pub struct Fibonacci {
    current: u128,
    after: u128,
}

pub fn reset(fib: &mut Fibonacci) {
    fib.current = 0;
    fib.after = 1;
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci {
            current: 0,
            after: 1,
        }
    }

    pub fn reset(&mut self) {
        self.current = 0;
        self.after = 1;
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

    #[test]
    fn test_iterator() {
        let mut fib = Fibonacci::new();

        let five: Vec<u128> = fib.take(5).collect();

        assert_eq!(five, vec![0, 1, 1, 2, 3]);
    }

    #[test]
    fn test_reset() {
        let mut fib = Fibonacci::new();
        reset(&mut fib);
    }

    #[test]
    fn test_it() {
        let a = [1, 2, 3, 4];
        let mut iter = a.into_iter();

        let result: Vec<&i32> = iter.by_ref().take_while(|n| **n != 3).collect();

        assert_eq!(result, &[&1, &2]);

        let result: Vec<&i32> = iter.collect();

        assert_eq!(result, &[&4]);
    }
}
