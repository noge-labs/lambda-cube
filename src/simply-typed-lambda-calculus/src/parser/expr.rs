use super::{
    error::ParserError,
    lexer::tokens::Token,
    macros::{consume, match_token},
    parsetree::{Abs, App, Arrow, Expr, Int, TInt, Type, Var},
    state::Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_variable_expr(&mut self) -> Result<Expr, ParserError> {
        let (token, range) = consume!(self, Token::Variable(var) => var.clone())?;

        Ok(Expr::Var(Var { value: token, range }))
    }

    pub fn parse_number_expr(&mut self) -> Result<Expr, ParserError> {
        let (token, range) = consume!(self, Token::Number(num) => num.clone())?;

        Ok(Expr::Int(Int { value: token, range }))
    }

    pub fn parse_atom(&mut self) -> Result<Expr, ParserError> {
        match self.get() {
            Token::LParen => self.parse_parens_expr(),
            Token::Variable(_) => self.parse_variable_expr(),
            Token::Number(_) => self.parse_number_expr(),
            _ => self.fail(),
        }
    }

    pub fn parse_abs(&mut self) -> Result<Expr, ParserError> {
        let (_, range) = consume!(self, Token::Lambda)?;
        let (param, _) = consume!(self, Token::Variable(var) => var.clone())?;

        consume!(self, Token::Colon)?;
        let param_type = self.parse_type()?;

        consume!(self, Token::Dot)?;
        let body = self.parse_expr()?;
        let endr = body.range();

        Ok(Expr::Abs(Abs {
            param,
            param_ty: param_type,
            body: Box::new(body),
            range: range.mix(endr),
        }))
    }

    pub fn parse_call(&mut self) -> Result<Expr, ParserError> {
        let lam = self.parse_atom()?;

        let mut args = Vec::new();

        while let Some(arg) = self.try_single(|state| state.parse_atom())? {
            args.push(arg);
        }

        if args.is_empty() {
            return Ok(lam);
        }

        let app = args.iter().fold(lam, |app, arg| {
            Expr::App(App {
                lambda: Box::new(app.clone()),
                argm: Box::new(arg.clone()),
                range: app.range().mix(arg.range()),
            })
        });

        Ok(app)
    }

    pub fn parse_application(&mut self) -> Result<Expr, ParserError> {
        let head = self.parse_call()?;

        Ok(head)
    }

    pub fn parse_parens_expr(&mut self) -> Result<Expr, ParserError> {
        consume!(self, Token::LParen)?;
        let expr = self.parse_expr()?;
        consume!(self, Token::RParen)?;

        Ok(expr)
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        match self.get() {
            Token::Lambda => self.parse_abs(),
            _ => self.parse_application(),
        }
    }

    pub fn parse_simple_type(&mut self) -> Result<Type, ParserError> {
        match self.get() {
            Token::TInt => {
                consume!(self, Token::TInt)?;
                Ok(Type::TInt(TInt {}))
            }
            Token::LParen => {
                consume!(self, Token::LParen)?;
                let ty = self.parse_type()?;
                consume!(self, Token::RParen)?;
                Ok(ty)
            }
            _ => self.fail(),
        }
    }

    pub fn parse_arrow_partial(&mut self, head: Type) -> Result<Type, ParserError> {
        if let Token::Arrow = self.get() {
            consume!(self, Token::Arrow)?;
            let body = self.parse_type()?;

            Ok(Type::Arrow(Arrow {
                left: Box::new(head.clone()),
                right: Box::new(body.clone()),
            }))
        } else {
            Ok(head)
        }
    }

    pub fn parse_type(&mut self) -> Result<Type, ParserError> {
        if let Token::LParen = self.get() {
            consume!(self, Token::LParen)?;
            let head = self.parse_type()?;
            consume!(self, Token::RParen)?;

            self.parse_arrow_partial(head)
        } else {
            let head = self.parse_simple_type()?;
            self.parse_arrow_partial(head)
        }
    }
}
