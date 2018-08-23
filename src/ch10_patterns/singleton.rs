//! # Singleton
//!
//! "Singleton in Rust" example from
//! https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
use std::mem;
use std::ptr;
use std::sync::{Arc, Mutex, Once};
static ONCE: Once = Once::new();

#[allow(dead_code)]
pub struct Solo {
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

pub fn singleton() -> Arc<Mutex<Solo>> {
    // Initialize it to a null value
    static mut SINGLETON: *const Arc<Mutex<Solo>> = ptr::null_mut();

    unsafe {
        ONCE.call_once(|| {
            SINGLETON = mem::transmute(Box::new(Arc::new(Mutex::new(Solo::new()))));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

    #[test]
    fn test_it_works() {
        let threads = (0..10).map(|_| {
            thread::spawn(move || {
                let s = singleton();
                let mut data = s.lock().unwrap();
                data.age = data.age + 1;
            })
        });

        threads.for_each(|t| t.join().expect("paniced"));

        {
            let s = singleton();
            let data = s.lock().unwrap();
            assert_eq!(data.age, 65u8);
            assert_eq!(data.name, String::from("Han"))
        }
    }
}
