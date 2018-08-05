use std::io::Cursor;
use std::io::Read;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
enum Token<'a> {
    EOF,
    Def,
    Extern,
    Identifier(&'a str),
    Number(f64),
    Illegal,
}

struct Lexer<'a> {
    buf: &'a str,
    buf_iter: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(buf: &'a str) -> Self {
        Lexer {
            buf: &buf,
            buf_iter: buf.char_indices().peekable(),
        }
    }

    fn read_identifier(&mut self, index: usize) -> Token<'a> {
        let rest = &self.buf[index..];
        match rest.split_whitespace().next() {
            Some("def") => {
                self.buf_iter.nth(2);
                Token::Def
            }
            Some("extern") => {
                self.buf_iter.nth(5);
                Token::Extern
            }
            Some(word) => {
                self.buf_iter.nth(word.len() - 1);
                Token::Identifier(word)
            }
            None => Token::Illegal,
        }
    }

    fn read_number(&mut self, index: usize) -> Token<'a> {
        let rest = &self.buf[index..];
        let word = rest.split_whitespace().next();

        word.and_then(|w| w.parse::<f64>().ok())
            .map_or(Token::Illegal, Token::Number)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self
            .buf_iter
            .peek()
            .map_or(false, |&(_, x)| x.is_whitespace())
        {
            self.buf_iter.next();
        }
        match self.buf_iter.peek() {
            Some(&(i, x)) if x.is_alphabetic() => Some(self.read_identifier(i)),
            Some(&(i, x)) if x.is_numeric() => Some(self.read_number(i)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let _lex = Lexer::new("hello");
    }

    #[test]
    fn test_it_parses_def() {
        let mut lex = Lexer::new("def");
        assert_eq!(lex.next(), Some(Token::Def));
    }

    #[test]
    fn test_it_parses_extern() {
        let mut lex = Lexer::new("extern");
        assert_eq!(lex.next(), Some(Token::Extern));
    }

    #[test]
    fn test_it_parses_identifier() {
        let mut lex = Lexer::new("rock");
        assert_eq!(lex.next(), Some(Token::Identifier("rock")));
    }

    #[test]
    fn test_it_parses_number() {
        let mut lex = Lexer::new("42.7");
        assert_eq!(lex.next(), Some(Token::Number(42.7f64)));
    }

    #[test]
    fn test_it_parses_multiple() {
        let mut lex = Lexer::new("def hello_world 42");
        assert_eq!(lex.next(), Some(Token::Def));
        assert_eq!(lex.next(), Some(Token::Identifier("hello_world")));
        assert_eq!(lex.next(), Some(Token::Number(42f64)));
    }

}
