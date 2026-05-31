use crate::lexer::{Delimiter, Literal, Operator, Span, Token};

pub enum Expr {
    Literal(Literal),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    UnaryOp {
        operator: Operator,
        operand: Box<Expr>,
    },
    Call {
        function: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Assignment {
        target: Box<Expr>,
        value: Box<Expr>,
    },
}

pub enum Stmt {
    Expr(Expr),
    Let {
        name: String,
        value: Expr,
    },
    Func {
        name: String,
        params: Vec<Param>,
        body: Box<Stmt>,
        ty: Type,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Loop {
        body: Box<Stmt>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Scope {
        statements: Vec<Stmt>,
    },
    Return {
        value: Option<Expr>,
    },
    Extern {
        name: String,
        params: Vec<Param>,
        ty: Type,
    },
    Break,
    Continue,
}

pub enum Type {
    Named(String),
    I8,
    I16,
    I32,
    I64,
    ISize,
    U8,
    U16,
    U32,
    U64,
    Usize,
    F32,
    F64,
    Pointer(Box<Type>),
    Bool,
    Unit,
}

pub struct Param {
    name: String,
    ty: Type,
}

pub struct Parser {
    tokens: Vec<(Token, Span)>,
    position: usize,
    scope_stack: Vec<Stmt>,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Span)>) -> Self {
        Self {
            tokens,
            position: 0,
            scope_stack: Vec::new(),
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(
            self.tokens.get(self.position).map(|x| &x.0),
            Some(Token::Eof) | None
        )
    }

    fn advance(&mut self) -> anyhow::Result<&(Token, Span)> {
        anyhow::ensure!(
            !self.is_at_end(),
            "Unexpected end of input at position {}",
            self.position
        );

        let token = self.tokens.get(self.position);
        self.position += 1;
        Ok(token.unwrap())
    }

    fn peek(&self) -> anyhow::Result<&(Token, Span)> {
        anyhow::ensure!(
            !self.is_at_end(),
            "Unexpected end of input at position {}",
            self.position
        );

        Ok(self.tokens.get(self.position).unwrap())
    }

    fn expect_delim(&mut self, expected: Delimiter) -> anyhow::Result<&(Token, Span)> {
        let (token, span) = self.peek()?;
        anyhow::ensure!(
            matches!(token, Token::Delimiter(delim) if *delim == expected),
            "Expected delimiter {:?} at {}, found {:?}",
            expected,
            span,
            token
        );

        self.advance()
    }

    fn parse_stmt(&mut self) -> anyhow::Result<Stmt> {
        // let (token, span) = self.peek()?;
        // match token {
        //     Token::Keyword(Keyword::Let) => self.parse_let(),
        //     Token::Keyword(Keyword::Func) => self.parse_func(),
        //     Token::Keyword(Keyword::If) => self.parse_if(),
        // }
        panic!()
    }

    pub fn parse(&mut self) -> anyhow::Result<()> {
        while !self.is_at_end() {
            let stmt = self.parse_stmt()?;
            self.scope_stack.push(stmt);
        }
        Ok(())
    }
}
