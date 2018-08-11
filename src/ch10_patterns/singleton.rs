//! # Singleton
//!
//! "Singleton in Rust" example from
//! https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
use std::mem;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};

struct Solo {
    name: String,
    age: u8,
}

impl Solo {
    fn new() -> Self {
        Solo {
            name: String::from("Han"),
            age: 55,
        }
    }
}

#[derive(Clone)]
struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Arc<Mutex<Solo>>,
}

fn singleton() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader {
                inner: Arc::new(Mutex::new(Solo::new())),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_works() {
        {
            let s = singleton();
            let mut data = s.inner.lock().unwrap();
            (*data).age = 10u8;
        }
        {
            let s = singleton();
            let data = s.inner.lock().unwrap();
            assert_eq!((*data).age, 10u8);
            assert_eq!((*data).name, String::from("Han"))
        }
    }
}
