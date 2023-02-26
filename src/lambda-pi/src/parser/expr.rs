use super::{
    error::ParserError,
    lexer::tokens::Token,
    macros::{consume, match_token},
    parsetree::{Abs, Anno, Appl, Checkable, Expr, Int, Prod, Star, Var},
    state::Parser,
    symbol::Symbol,
};

impl<'a> Parser<'a> {
    pub fn parse_variable_expr(&mut self) -> Result<Expr, ParserError> {
        let (token, range) = consume!(self, Token::Variable(var) => var.clone())?;
        let symbol = Symbol::new(token);

        Ok(Expr::Var(Var { value: symbol, range }))
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

    pub fn parse_parens_expr(&mut self) -> Result<Expr, ParserError> {
        consume!(self, Token::LParen)?;
        let expr = self.parse_expr()?;
        consume!(self, Token::RParen)?;

        Ok(expr)
    }

    pub fn parse_abs(&mut self) -> Result<Checkable, ParserError> {
        let (_, range) = consume!(self, Token::Lambda)?;
        let (param, _) = consume!(self, Token::Variable(var) => var.clone())?;
        let symbol = Symbol::new(param);

        consume!(self, Token::Colon)?;
        let param_type = self.parse_checkable()?;

        consume!(self, Token::Dot)?;
        let body = self.parse_checkable()?;

        Ok(Checkable::Abs(Abs {
            param: symbol,
            param_ty: Box::new(param_type),
            body: Box::new(body),
            range,
        }))
    }

    pub fn parse_call(&mut self) -> Result<Expr, ParserError> {
        let func = self.parse_atom()?;
        let mut args = Vec::new();

        while let Some(arg) = self.try_single(|state| state.parse_checkable())? {
            args.push(arg);
        }

        if !args.is_empty() {
            let appl = args.iter().fold(func, |fun, arg| {
                Expr::Appl(Appl {
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

    pub fn parse_appl(&mut self) -> Result<Expr, ParserError> {
        let head = self.parse_call()?;

        Ok(head)
    }

    pub fn parse_annot(&mut self) -> Result<Expr, ParserError> {
        let (_, lpos) = consume!(self, Token::LParen)?;
        let expr = self.parse_checkable()?;
        consume!(self, Token::Colon)?;
        let anno = self.parse_checkable()?;
        let (_, rpos) = consume!(self, Token::RParen)?;

        Ok(Expr::Anno(Anno { expr, anno, range: lpos.mix(rpos) }))
    }

    pub fn parse_pi(&mut self) -> Result<Expr, ParserError> {
        let (_, range) = consume!(self, Token::Pi)?;
        let (param, _) = consume!(self, Token::Variable(var) => var.clone())?;
        let symbol = Symbol::new(param);

        consume!(self, Token::Colon)?;
        let param_ty = self.parse_checkable()?;

        consume!(self, Token::Dot)?;
        let body = self.parse_checkable()?;

        Ok(Expr::Prod(Prod {
            param: symbol,
            param_ty: Box::new(param_ty),
            body: Box::new(body.clone()),
            range: range.mix(body.range()),
        }))
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        match self.get() {
            Token::LParen => self.parse_annot(),
            Token::Pi => self.parse_pi(),
            Token::Star => self.parse_kind(),
            _ => self.parse_appl(),
        }
    }

    pub fn parse_checkable(&mut self) -> Result<Checkable, ParserError> {
        match self.get() {
            Token::Lambda => self.parse_abs(),
            _ => self.parse_inf(),
        }
    }

    pub fn parse_inf(&mut self) -> Result<Checkable, ParserError> {
        let inf = self.parse_expr()?;

        Ok(Checkable::Inf(Box::new(inf)))
    }

    pub fn parse_kind(&mut self) -> Result<Expr, ParserError> {
        match self.get() {
            Token::Star => {
                let (_, range) = consume!(self, Token::Star)?;
                Ok(Expr::Star(Star { range }))
            }
            _ => self.fail(),
        }
    }
}
