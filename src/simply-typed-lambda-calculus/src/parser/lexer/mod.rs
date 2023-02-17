use self::{state::Lexer, tokens::Token};
use super::location::Range;

pub mod state;
pub mod tokens;

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r')
}

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}

fn is_reserved(c: char) -> bool {
    matches!(c, '(' | ')' | '.' | ':' | 'λ')
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
            "int" => Token::TInt,
            "->" => Token::Arrow,
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
                ':' => self.single_token(Token::Colon, start),
                '.' => self.single_token(Token::Dot, start),
                chr if is_digit(*chr) => {
                    let num = self.accu_while(is_digit);
                    let num = num.parse::<usize>().unwrap();
                    let tok = Token::Number(num);
                    self.make_token(tok, start)
                }
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
