//! # Factory
//! Basically just a fn on an impl
struct Customer {
    name: String,
}

impl Customer {
    fn new() -> Self {
        Customer {
            name: String::new(),
        }
    }

    fn from_defaults() -> Self {
        Customer {
            name: String::from("Factory Worker"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ctor() {
        let cust = Customer::new();
        assert_eq!(cust.name, "".to_string())
    }

    #[test]
    fn test_defaults() {
        let defaulted = Customer::from_defaults();
        assert_eq!(defaulted.name, "Factory Worker".to_string())
    }

}
