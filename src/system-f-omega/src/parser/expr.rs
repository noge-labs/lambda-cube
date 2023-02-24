use super::{
    error::ParserError,
    lexer::tokens::Token,
    location::Range,
    macros::{consume, match_token},
    parsetree::{
        Abs, Anno, App, Arrow, Expr, Forall, Int, Kind, KindAlias, KindArrow, KindVar, LetAlias,
        Star, TAbs, TApp, TInt, TVar, TyAbs, TyAnno, TyApp, Type, TypeAlias, Var,
    },
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

    pub fn parse_abs_expr(&mut self, range: Range) -> Result<Expr, ParserError> {
        let (param, _) = consume!(self, Token::Variable(var) => var.clone())?;
        let symbol = Symbol::new(param);

        consume!(self, Token::Colon)?;
        let param_type = self.parse_type()?;

        consume!(self, Token::Dot)?;
        let body = self.parse_expr()?;
        let endr = body.range();

        Ok(Expr::Abs(Abs {
            param: symbol,
            param_ty: param_type,
            body: Box::new(body),
            range: range.mix(endr),
        }))
    }

    pub fn parse_abs_type(&mut self, range: Range) -> Result<Expr, ParserError> {
        let (param, _) = consume!(self, Token::TVar(var) => var.clone())?;
        let symbol = Symbol::new(param);

        consume!(self, Token::Colon)?;
        let param_type = self.parse_kind()?;

        consume!(self, Token::Dot)?;
        let body = self.parse_expr()?;
        let endr = body.range();

        Ok(Expr::TAbs(TAbs {
            param: symbol,
            param_ty: param_type,
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

    pub fn parse_let_alias(&mut self) -> Result<Expr, ParserError> {
        let (_, range) = consume!(self, Token::Let)?;
        let (name, _) = consume!(self, Token::Variable(name) => name.clone())?;
        let symbol = Symbol::new(name);

        consume!(self, Token::Colon)?;
        let anno = self.parse_type()?;

        consume!(self, Token::Equal)?;
        let value = self.parse_expr()?;

        consume!(self, Token::In)?;
        let body = self.parse_expr()?;

        Ok(Expr::LetAlias(LetAlias {
            name: symbol,
            value: Box::new(Expr::Anno(Anno {
                expr: Box::new(value.clone()),
                anno,
                range: range.mix(value.range()),
            })),
            body: Box::new(body.clone()),
            range: range.mix(body.range()),
        }))
    }

    pub fn parse_type_alias(&mut self) -> Result<Expr, ParserError> {
        let (_, range) = consume!(self, Token::Type)?;
        let (name, _) = consume!(self, Token::TVar(name) => name.clone())?;
        let symbol = Symbol::new(name);

        consume!(self, Token::Colon)?;
        let anno = self.parse_kind()?;

        consume!(self, Token::Equal)?;
        let value = self.parse_type()?;

        consume!(self, Token::In)?;
        let body = self.parse_expr()?;

        Ok(Expr::TypeAlias(TypeAlias {
            name: symbol,
            value: Type::TyAnno(TyAnno { ty: Box::new(value.clone()), anno }),
            body: Box::new(body.clone()),
            range: range.mix(body.range()),
        }))
    }

    pub fn parse_kind_alias(&mut self) -> Result<Expr, ParserError> {
        let (_, range) = consume!(self, Token::Kind)?;
        let (name, _) = consume!(self, Token::TVar(name) => name.clone())?;
        let symbol = Symbol::new(name);

        consume!(self, Token::Equal)?;
        let value = self.parse_kind()?;

        consume!(self, Token::In)?;
        let body = self.parse_expr()?;

        Ok(Expr::KindAlias(KindAlias {
            name: symbol,
            body: Box::new(body.clone()),
            value: value.clone(),
            range: range.mix(body.range()),
        }))
    }

    pub fn parse_annot(&mut self) -> Result<Expr, ParserError> {
        let (_, range) = consume!(self, Token::LParen)?;
        let expr = self.parse_annot_lambda()?;

        consume!(self, Token::Colon)?;
        let ty = self.parse_type()?;

        Ok(Expr::Anno(Anno { expr: Box::new(expr), anno: ty, range }))
    }

    pub fn parse_annot_lambda(&mut self) -> Result<Expr, ParserError> {
        match self.get() {
            Token::Lambda => self.parse_abs(),
            _ => self.parse_application(),
        }
    }

    pub fn parse_annot_expr(&mut self) -> Result<Expr, ParserError> {
        match self.get() {
            Token::LParen => self.parse_annot(),
            _ => self.parse_expr(),
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        match self.get() {
            Token::Let => self.parse_let_alias(),
            Token::Type => self.parse_type_alias(),
            Token::Kind => self.parse_kind_alias(),
            _ => self.parse_annot_lambda(),
        }
    }

    pub fn parse_type_call(&mut self) -> Result<Type, ParserError> {
        let func = self.parse_type()?;
        let mut args = Vec::new();

        while let Some(arg) = self.try_single(|state| state.parse_type())? {
            args.push(arg);
        }

        if !args.is_empty() {
            let appl = args.iter().fold(func, |fun, arg| {
                Type::TyApp(TyApp {
                    lambda: Box::new(fun.clone()),
                    argm: Box::new(arg.clone()),
                })
            });

            Ok(appl)
        } else {
            Ok(func)
        }
    }

    pub fn parse_type_application(&mut self) -> Result<Type, ParserError> {
        let head = self.parse_type_call()?;

        Ok(head)
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
                Ok(Type::TInt(TInt {}))
            }
            Token::TVar(_) => {
                let (token, _) = consume!(self, Token::TVar(var) => var.clone())?;
                let symbol = Symbol::new(token);

                Ok(Type::TVar(TVar { value: symbol }))
            }
            Token::Forall => {
                consume!(self, Token::Forall)?;
                let (token, _) = consume!(self, Token::TVar(var) => var.clone())?;
                let symbol = Symbol::new(token);

                consume!(self, Token::Colon)?;
                let param_type = self.parse_kind()?;

                consume!(self, Token::Dot)?;
                let body = self.parse_type()?;

                Ok(Type::Forall(Forall {
                    param: symbol,
                    param_ty: param_type,
                    body: Box::new(body),
                }))
            }
            Token::Lambda => {
                consume!(self, Token::Lambda)?;
                let (param, _) = consume!(self, Token::TVar(var) => var.clone())?;
                let symbol = Symbol::new(param);

                consume!(self, Token::Colon)?;
                let param_type = self.parse_kind()?;

                consume!(self, Token::Dot)?;
                let body = self.parse_type()?;

                Ok(Type::TyAbs(TyAbs {
                    param: symbol,
                    param_ty: param_type,
                    body: Box::new(body),
                }))
            }
            _ => self.parse_type_application(),
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

    pub fn parse_arrow_kind(&mut self, head: Kind) -> Result<Kind, ParserError> {
        if let Token::Arrow = self.get() {
            consume!(self, Token::Arrow)?;
            let body = self.parse_kind()?;

            Ok(Kind::KindArrow(KindArrow {
                left: Box::new(head.clone()),
                right: Box::new(body.clone()),
            }))
        } else {
            Ok(head)
        }
    }

    pub fn parse_simple_kind(&mut self) -> Result<Kind, ParserError> {
        match self.get() {
            Token::Star => {
                consume!(self, Token::Star)?;
                Ok(Kind::Star(Star {}))
            }
            Token::TVar(_) => {
                let (token, _) = consume!(self, Token::TVar(var) => var.clone())?;
                let symbol = Symbol::new(token);

                Ok(Kind::KindVar(KindVar { value: symbol }))
            }
            _ => self.fail(),
        }
    }

    pub fn parse_kind(&mut self) -> Result<Kind, ParserError> {
        if let Token::LParen = self.get() {
            consume!(self, Token::LParen)?;
            let head = self.parse_kind()?;
            consume!(self, Token::RParen)?;

            self.parse_arrow_kind(head)
        } else {
            let head = self.parse_simple_kind()?;
            self.parse_arrow_kind(head)
        }
    }
}
