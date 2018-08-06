use std::io::Read;

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
}

pub fn get_token<R: Read>(input: &mut R) -> Token {
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
    use std::io::Cursor;

    #[test]
    fn it_parses_def() {
        let mut input = Cursor::new(b"def hello");
        let result = get_token(&mut input);
        assert_eq!(result, Token::Def);
    }

    #[test]
    fn it_parses_extern() {
        let mut input = Cursor::new(b"extern hello");
        let result = get_token(&mut input);
        assert_eq!(result, Token::Extern);
    }

    #[test]
    fn it_parses_anything_else() {
        let mut input = Cursor::new(b"hello");
        let result = get_token(&mut input);
        assert_eq!(result, Token::Identifier(String::from("hello")));
    }

    #[test]
    fn it_parses_multiple() {
        let mut input = Cursor::new(b"def hello");
        assert_eq!(get_token(&mut input), Token::Def);
        assert_eq!(
            get_token(&mut input),
            Token::Identifier(String::from("hello"))
        );
        assert_eq!(get_token(&mut input), Token::EOF);
    }

    #[test]
    fn it_parses_number() {
        let mut input = Cursor::new(b"32.4");
        let result = get_token(&mut input);
        assert_eq!(result, Token::Number(32.4));
    }

}
