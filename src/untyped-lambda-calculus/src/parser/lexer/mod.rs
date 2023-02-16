use self::{state::Lexer, tokens::Token};
use super::location::Range;

pub mod state;
pub mod tokens;

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r')
}

fn is_reserved(c: char) -> bool {
    matches!(c, '(' | ')' | '.' | 'λ')
}

fn is_valid_char(c: char) -> bool {
    !is_reserved(c) && !is_whitespace(c)
}

impl<'a> Lexer<'a> {
    pub fn single_token(&mut self, token: Token, start: usize) -> (Token, Range) {
        self.next_char();
        self.make_token(token, start)
    }

    fn to_keyword(buf: &str) -> Token {
        match buf {
            "lambda" => Token::Lambda,
            _ => Token::Variable(buf.to_string()),
        }
    }

    pub fn lex_token(&mut self) -> (Token, Range) {
        let start = self.span();
        let charp = self.peekable.peek();

        match charp {
            None => self.make_token(Token::Eof, start),
            Some(c) => match c {
                chr if is_whitespace(*chr) => {
                    self.accu_while(is_whitespace);
                    self.lex_token()
                }
                '\n' => {
                    self.accu_while(|x| x == '\n' || x == '\r');
                    self.lex_token()
                }
                'λ' => self.single_token(Token::Lambda, start),
                '(' => self.single_token(Token::LParen, start),
                ')' => self.single_token(Token::RParen, start),
                '.' => self.single_token(Token::Dot, start),
                chr if is_valid_char(*chr) => {
                    let str = self.accu_while(is_valid_char);
                    let tok = Lexer::to_keyword(str);
                    self.make_token(tok, start)
                }
                _ => self.single_token(Token::Error, start),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::lexer::tokens::Token;

    fn test_lex(expr: &str, tokens: Vec<Token>) -> bool {
        let mut lexer = crate::parser::Lexer::new(expr);

        std::iter::from_fn(|| {
            let (token, _) = lexer.lex_token();
            if token == Token::Error {
                return None;
            }
            Some(token)
        })
        .zip(tokens)
        .all(|(input, expected)| input == expected)
    }

    #[test]
    fn test_lex_variable() {
        let received = "x";
        let expected = vec![Token::Variable(String::from("x"))];

        assert!(test_lex(received, expected))
    }

    #[test]
    fn test_lex_lambda() {
        let received = "λid. id";
        let expected = vec![
            Token::Lambda,
            Token::Variable(String::from("id")),
            Token::Dot,
            Token::Variable(String::from("id")),
        ];

        assert!(test_lex(received, expected))
    }

    #[test]
    fn test_lex_parens() {
        let received = "(x) y";
        let expected = vec![
            Token::LParen,
            Token::Variable(String::from("x")),
            Token::RParen,
            Token::Variable(String::from("y")),
        ];

        assert!(test_lex(received, expected))
    }

    #[test]
    fn test_lex_nested_parens() {
        let received = "((()()))";
        let expected = vec![
            Token::LParen,
            Token::LParen,
            Token::LParen,
            Token::RParen,
            Token::LParen,
            Token::RParen,
            Token::RParen,
            Token::RParen,
        ];

        assert!(test_lex(received, expected))
    }
}
