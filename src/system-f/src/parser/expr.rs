use super::{
    error::ParserError,
    lexer::tokens::Token,
    location::Range,
    macros::{consume, match_token},
    parsetree::{Abs, App, Expr, Int, TAbs, TApp, Type, Var},
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

    pub fn parse_abs_expr(&mut self, range: Range) -> Result<Expr, ParserError> {
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

    pub fn parse_abs_type(&mut self, range: Range) -> Result<Expr, ParserError> {
        let (param, _) = consume!(self, Token::TVar(var) => var.clone())?;

        consume!(self, Token::Dot)?;
        let body = self.parse_expr()?;
        let endr = body.range();

        Ok(Expr::TAbs(TAbs {
            param,
            body: Box::new(body),
            range: range.mix(endr),
        }))
    }

    pub fn parse_abs(&mut self) -> Result<Expr, ParserError> {
        let (_, range) = consume!(self, Token::Lambda)?;

        if let Token::Variable(_) = self.get() {
            self.parse_abs_expr(range)
        } else {
            self.parse_abs_type(range)
        }
    }

    pub fn parse_type_arg(&mut self) -> Result<(Range, Type), ParserError> {
        let (_, range_l) = consume!(self, Token::LBracket)?;
        let argument_ty = self.parse_type()?;
        let (_, range_r) = consume!(self, Token::RBracket)?;

        Ok((range_l.mix(range_r), argument_ty))
    }

    pub fn parse_call(&mut self) -> Result<Expr, ParserError> {
        let mut func = self.parse_atom()?;
        let mut args = Vec::new();

        if let Token::LBracket = self.get() {
            while let Ok((loc, arg)) = self.parse_type_arg() {
                func = Expr::TApp(TApp {
                    lambda: Box::new(func.clone()),
                    argm: arg,
                    range: func.range().mix(loc),
                });
            }
        }

        while let Some(arg) = self.try_single(|state| state.parse_atom())? {
            args.push(arg);
        }

        if !args.is_empty() {
            let appl = args.iter().fold(func, |fun, arg| {
                Expr::App(App {
                    lambda: Box::new(fun.clone()),
                    argm: Box::new(arg.clone()),
                    range: fun.range().mix(arg.range()),
                })
            });

            Ok(appl)
        } else {
            Ok(func)
        }
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

    pub fn parse_let(&mut self) -> Result<Expr, ParserError> {
        let (_, range) = consume!(self, Token::Let)?;
        let (param, _) = consume!(self, Token::Variable(name) => name.clone())?;

        consume!(self, Token::Colon)?;
        let param_ty = self.parse_type()?;

        consume!(self, Token::Equal)?;
        let value = self.parse_expr()?;

        consume!(self, Token::In)?;
        let body = self.parse_expr()?;

        let func = Expr::Abs(Abs {
            param,
            param_ty,
            body: Box::new(body.clone()),
            range: range.mix(body.range()),
        });

        Ok(Expr::App(App {
            lambda: Box::new(func),
            argm: Box::new(value.clone()),
            range: range.mix(value.range()),
        }))
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        match self.get() {
            Token::Lambda => self.parse_abs(),
            Token::Let => self.parse_let(),
            _ => self.parse_application(),
        }
    }

    pub fn parse_simple_type(&mut self) -> Result<Type, ParserError> {
        match self.get() {
            Token::LParen => {
                consume!(self, Token::LParen)?;
                let ty = self.parse_type()?;
                consume!(self, Token::RParen)?;
                Ok(ty)
            }
            Token::TInt => {
                consume!(self, Token::TInt)?;
                Ok(Type::TInt)
            }
            Token::TVar(_) => {
                let (token, _) = consume!(self, Token::TVar(var) => var.clone())?;
                Ok(Type::TVar { value: token })
            }
            Token::Forall => {
                consume!(self, Token::Forall)?;
                let (token, _) = consume!(self, Token::TVar(var) => var.clone())?;
                consume!(self, Token::Dot)?;
                let body = self.parse_type()?;

                Ok(Type::Forall { param: token, body: Box::new(body) })
            }
            _ => self.fail(),
        }
    }

    pub fn parse_arrow_partial(&mut self, head: Type) -> Result<Type, ParserError> {
        if let Token::Arrow = self.get() {
            consume!(self, Token::Arrow)?;
            let body = self.parse_type()?;

            Ok(Type::Arrow {
                left: Box::new(head.clone()),
                right: Box::new(body.clone()),
            })
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
