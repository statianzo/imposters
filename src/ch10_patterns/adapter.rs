//! # Adapter Pattern
//! Getting something different to fit into an interface.
//! In Rust, this is accomplished by making an impl for a trait.
//! This example makes Mp4File and OggFile structs impl a
//! common CaptionedAudio trait.
struct Mp4File {}
impl Mp4File {
    fn track_words(&self) -> String {
        "Hey I just met you".to_owned()
    }
}

struct OggFile {}
impl OggFile {
    fn retrieve_lyrics(&self) -> String {
        "But here's my number".to_owned()
    }
}

trait CaptionedAudio {
    fn lyrics(&self) -> String;
}

impl CaptionedAudio for Mp4File {
    fn lyrics(&self) -> String {
        self.track_words()
    }
}

impl CaptionedAudio for OggFile {
    fn lyrics(&self) -> String {
        self.retrieve_lyrics()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_it_works() {
        let mp4 = Mp4File {};
        let ogg = OggFile {};
        assert_eq!(mp4.lyrics(), "Hey I just met you".to_owned());
        assert_eq!(ogg.lyrics(), "But here's my number".to_owned());
    }
}
