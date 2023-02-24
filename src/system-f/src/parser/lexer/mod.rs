use self::{state::Lexer, tokens::Token};
use super::location::Range;

pub mod state;
pub mod tokens;

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}

fn is_reserved(c: char) -> bool {
    matches!(c, '(' | ')' | '[' | ']' | '.' | ':' | '=' | 'λ')
}

fn is_valid_char(c: char) -> bool {
    !is_reserved(c) && !is_whitespace(c)
}

fn is_valid_upper_char(c: char) -> bool {
    is_valid_char(c) && c.is_uppercase()
}

impl<'a> Lexer<'a> {
    pub fn single_token(&mut self, token: Token, start: usize) -> (Token, Range) {
        self.next_char();
        self.make_token(token, start)
    }

    fn to_type(buf: &str) -> Token {
        match buf {
            "Int" => Token::TInt,
            _ => Token::TVar(buf.to_string()),
        }
    }

    fn to_keyword(buf: &str) -> Token {
        match buf {
            "lambda" => Token::Lambda,
            "forall" => Token::Forall,
            "let" => Token::Let,
            "in" => Token::In,
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
                '∀' => self.single_token(Token::Forall, start),
                'Λ' => self.single_token(Token::Forall, start),
                '=' => self.single_token(Token::Equal, start),
                '(' => self.single_token(Token::LParen, start),
                ')' => self.single_token(Token::RParen, start),
                '[' => self.single_token(Token::LBracket, start),
                ']' => self.single_token(Token::RBracket, start),
                ':' => self.single_token(Token::Colon, start),
                '.' => self.single_token(Token::Dot, start),
                chr if is_digit(*chr) => {
                    let num = self.accu_while(is_digit);
                    let num = num.parse::<usize>().unwrap();
                    let tok = Token::Number(num);
                    self.make_token(tok, start)
                }
                chr if is_valid_upper_char(*chr) => {
                    let str = self.accu_while(is_valid_char);
                    let tok = Lexer::to_type(str);
                    self.make_token(tok, start)
                }
                chr if is_valid_char(*chr) => {
                    let str = self.accu_while(is_valid_char);
                    let tok = Lexer::to_keyword(str);
                    self.make_token(tok, start)
                }
                _ => {
                    println!("error at: {:?}", c);
                    self.single_token(Token::Error, start)
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::lexer::tokens::Token;

    fn test_lex(expr: &str, tokens: Vec<Token>) -> Vec<(Token, Token)> {
        let mut lexer = crate::parser::Lexer::new(expr);
        let mut token_pairs = Vec::new();

        for expected_token in tokens {
            let (actual_token, _) = lexer.lex_token();
            token_pairs.push((actual_token, expected_token));
        }

        token_pairs
    }

    #[test]
    fn test_lex_variable() {
        let received = "[Int]";
        let expected = vec![Token::LBracket, Token::TInt, Token::RBracket];

        for (fst, snd) in test_lex(received, expected) {
            assert_eq!(fst, snd)
        }
    }

    #[test]
    fn test_lex_forall() {
        let received = "∀X. X -> X";
        let expected = vec![
            Token::Forall,
            Token::TVar(String::from("X")),
            Token::Dot,
            Token::TVar(String::from("X")),
            Token::Arrow,
            Token::TVar(String::from("X")),
        ];

        for (fst, snd) in test_lex(received, expected) {
            assert_eq!(fst, snd)
        }
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

        for (fst, snd) in test_lex(received, expected) {
            assert_eq!(fst, snd)
        }
    }

    #[test]
    fn test_lex_lambda_type() {
        let received = "λf: Int -> Int. f";
        let expected = vec![
            Token::Lambda,
            Token::Variable(String::from("f")),
            Token::Colon,
            Token::TInt,
            Token::Arrow,
            Token::TInt,
            Token::Dot,
            Token::Variable(String::from("f")),
        ];

        for (fst, snd) in test_lex(received, expected) {
            assert_eq!(fst, snd)
        }
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

        for (fst, snd) in test_lex(received, expected) {
            assert_eq!(fst, snd)
        }
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

        for (fst, snd) in test_lex(received, expected) {
            assert_eq!(fst, snd)
        }
    }
}
