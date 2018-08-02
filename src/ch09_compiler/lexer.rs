use std::io::Cursor;
use std::io::Read;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
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

    fn read_identifier(&mut self, index: usize) -> Option<Token> {
        let rest = &self.buf[index..];
        match rest.split_whitespace().next() {
            Some("def") => Some(Token::Def),
            _ => None,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self.buf_iter.clone();

        match iter.peek() {
            Some((i, x)) if x.is_alphabetic() => self.read_identifier(i.clone()),
            _ => None,
        }
    }
}

fn get_token<R: Read>(input: &mut R) -> Token {
    let mut chars = input.bytes().map(|b| char::from(b.unwrap()));

    let last_result = chars.next();
    if last_result.is_none() {
        return Token::EOF;
    }

    let last_char = last_result.unwrap();

    if last_char.is_alphabetic() {
        let mut id = String::new();
        id.push(last_char);

        let tail = chars
            .take_while(|c| c.is_alphanumeric())
            .collect::<String>();
        id.push_str(&tail);

        return match id.as_ref() {
            "def" => Token::Def,
            "extern" => Token::Extern,
            _ => Token::Identifier(id),
        };
    }

    if last_char.is_numeric() {
        let mut num = String::new();
        num.push(last_char);

        let tail = chars
            .take_while(|c| c.is_numeric() || c == &'.')
            .collect::<String>();
        num.push_str(&tail);

        return Token::Number(num.parse::<f64>().unwrap());
    }

    Token::EOF
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

}
