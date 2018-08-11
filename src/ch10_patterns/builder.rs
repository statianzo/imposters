//! # Builder pattern
//! This is bad. Just use String::push_str
//! Same thing that String::collect and String::extend do
//! https://doc.rust-lang.org/src/alloc/string.rs.html#1758-1764

pub struct StringBuilder<'a> {
    segments: Vec<&'a str>,
}

impl<'a> StringBuilder<'a> {
    pub fn new() -> Self {
        StringBuilder { segments: vec![] }
    }

    pub fn append(mut self, segment: &'a str) -> Self {
        self.segments.push(&segment);
        self
    }
}

impl<'a> From<StringBuilder<'a>> for String {
    fn from(builder: StringBuilder<'a>) -> Self {
        builder.segments.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_builds() {
        let b = StringBuilder::new()
            .append("This ")
            .append("could be ")
            .append("very long ")
            .append("and blow up ")
            .append("your program");

        assert_eq!(
            String::from(b),
            String::from("This could be very long and blow up your program"),
        )
    }
}
