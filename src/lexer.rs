#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

pub struct Lexer {
    source: String,
    position: usize,
    line: usize,
    col: usize,
    pub tokens: Vec<(Token, Span)>,
}

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Literal(Literal),
    Operator(Operator),
    Delimiter(Delimiter),
    Eof,
}

#[derive(Debug, Clone)]
pub enum Keyword {
    As,
    Break,
    Const,
    Continue,
    Else,
    Enum,
    Extern,
    Fn,
    For,
    If,
    In,
    Let,
    Loop,
    Match,
    Mod,
    Static,
    Struct,
    Type,
    Use,
    While,
}

impl Keyword {
    pub fn from(s: &str) -> anyhow::Result<Self> {
        match s {
            "as" => Ok(Self::As),
            "break" => Ok(Self::Break),
            "const" => Ok(Self::Const),
            "continue" => Ok(Self::Continue),
            "else" => Ok(Self::Else),
            "enum" => Ok(Self::Enum),
            "extern" => Ok(Self::Extern),
            "fn" => Ok(Self::Fn),
            "for" => Ok(Self::For),
            "if" => Ok(Self::If),
            "in" => Ok(Self::In),
            "let" => Ok(Self::Let),
            "loop" => Ok(Self::Loop),
            "match" => Ok(Self::Match),
            "mod" => Ok(Self::Mod),
            "static" => Ok(Self::Static),
            "struct" => Ok(Self::Struct),
            "type" => Ok(Self::Type),
            "use" => Ok(Self::Use),
            "while" => Ok(Self::While),
            _ => anyhow::bail!("Invalid keyword: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Numeric(String),
    Boolean(bool),
    String(String),
    Char(char),
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Plus,         // +
    Minus,        // -
    Asterisk,     // *
    Slash,        // /
    Percent,      // %
    Assign,       // =
    Equal,        // ==
    NotEqual,     // !=
    Greater,      // >
    Less,         // <
    GreaterEqual, // >=
    LessEqual,    // <=
    LogicalAnd,   // &&
    LogicalOr,    // ||
    Exclem,       // !
    Ampersand,    // &
    Pipe,         // |
    Xor,          // ^
    LShift,       // <<
    RShift,       // >>
    AddAssign,    // +=
    SubAssign,    // -=
    MulAssign,    // *=
    DivAssign,    // /=
    ModAssign,    // %=
    BitAndAssign, // &=
    BitOrAssign,  // |=
    BitXorAssign, // ^=
    LShiftAssign, // <<=
    RShiftAssign, // >>=
    Question,     // ?
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Delimiter {
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    LAngle,      // < generics, reserved for future use
    RAngle,      // > generics, reserved for future use
    Semicolon,   // ;
    Colon,       // :
    DoubleColon, // ::
    ThinArrow,   // ->
    ThickArrow,  // =>
    Comma,       // ,
    Dot,         // .
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            position: 0,
            line: 1,
            col: 1,
            tokens: Vec::new(),
        }
    }

    fn advance(&mut self, chars: &[char], amount: usize) {
        for _ in 0..amount {
            if let Some(&c) = chars.get(self.position) {
                if c == '\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }
                self.position += 1;
            }
        }
    }

    pub fn tokenize(&mut self) -> anyhow::Result<()> {
        let chars = self.source.chars().collect::<Vec<_>>();

        while let Some(&c) = chars.get(self.position) {
            let span = Span {
                line: self.line,
                col: self.col,
            };

            self.advance(&chars, 1);

            let token = match c {
                ' ' | '\r' | '\n' | '\t' => {
                    continue;
                }

                '(' => Token::Delimiter(Delimiter::LParen),
                ')' => Token::Delimiter(Delimiter::RParen),
                '{' => Token::Delimiter(Delimiter::LBrace),
                '}' => Token::Delimiter(Delimiter::RBrace),
                '[' => Token::Delimiter(Delimiter::LBracket),
                ']' => Token::Delimiter(Delimiter::RBracket),
                ';' => Token::Delimiter(Delimiter::Semicolon),
                ':' => {
                    if chars.get(self.position) == Some(&':') {
                        self.advance(&chars, 1);
                        Token::Delimiter(Delimiter::DoubleColon)
                    } else {
                        Token::Delimiter(Delimiter::Colon)
                    }
                }
                ',' => Token::Delimiter(Delimiter::Comma),
                '.' => Token::Delimiter(Delimiter::Dot),
                '=' => match chars.get(self.position) {
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::Equal)
                    }
                    Some(&'>') => {
                        self.advance(&chars, 1);
                        Token::Delimiter(Delimiter::ThickArrow)
                    }
                    _ => Token::Operator(Operator::Assign),
                },
                '-' => match chars.get(self.position) {
                    Some(&'>') => {
                        self.advance(&chars, 1);
                        Token::Delimiter(Delimiter::ThinArrow)
                    }
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::SubAssign)
                    }
                    Some(&first_digit @ '0'..='9') => {
                        let can_start_signed_number = match self.tokens.last() {
                            None => true,
                            Some((token, _)) => match token {
                                Token::Operator(_) | Token::Keyword(_) => true,
                                Token::Delimiter(delim) => matches!(
                                    delim,
                                    Delimiter::LParen
                                        | Delimiter::LBrace
                                        | Delimiter::LBracket
                                        | Delimiter::Comma
                                        | Delimiter::Colon
                                        | Delimiter::DoubleColon
                                        | Delimiter::ThinArrow
                                        | Delimiter::ThickArrow
                                        | Delimiter::Semicolon
                                ),
                                _ => false,
                            },
                        };

                        if !can_start_signed_number {
                            Token::Operator(Operator::Minus)
                        } else {
                            self.advance(&chars, 1);
                            let mut numeric_literal = String::from("-");
                            numeric_literal.push(first_digit);

                            if first_digit == '0'
                                && let Some(&prefix) = chars.get(self.position)
                                && matches!(prefix, 'x' | 'b' | 'o')
                            {
                                self.advance(&chars, 1);
                                numeric_literal.push(prefix);
                                let validator: fn(char) -> bool = match prefix {
                                    'x' => |ch| ch.is_ascii_hexdigit(),
                                    'b' => |ch| ch == '0' || ch == '1',
                                    'o' => |ch| ('0'..='7').contains(&ch),
                                    _ => unreachable!(),
                                };
                                while let Some(&digit) = chars.get(self.position) {
                                    if validator(digit) {
                                        self.advance(&chars, 1);
                                        numeric_literal.push(digit);
                                    } else if digit == '_' {
                                        self.advance(&chars, 1);
                                    } else if digit.is_ascii_whitespace()
                                        || matches!(
                                            digit,
                                            '(' | ')'
                                                | '{'
                                                | '}'
                                                | '['
                                                | ']'
                                                | ';'
                                                | ':'
                                                | ','
                                                | '.'
                                                | '+'
                                                | '-'
                                                | '*'
                                                | '/'
                                                | '%'
                                                | '='
                                                | '!'
                                                | '<'
                                                | '>'
                                                | '&'
                                                | '|'
                                                | '^'
                                                | '?'
                                                | '\''
                                                | '"'
                                        )
                                    {
                                        break;
                                    } else {
                                        anyhow::bail!(
                                            "Invalid {} literal: {}{} at {}",
                                            match prefix {
                                                'x' => "hexadecimal",
                                                'b' => "binary",
                                                'o' => "octal",
                                                _ => unreachable!(),
                                            },
                                            numeric_literal,
                                            digit,
                                            span
                                        );
                                    }
                                }

                                Token::Literal(Literal::Numeric(numeric_literal))
                            } else {
                                let mut has_dot = false;
                                while let Some(&next_char) = chars.get(self.position) {
                                    if matches!(
                                        next_char,
                                        'd' | 'D' | 'f' | 'F' | 'u' | 'U' | 'l' | 'L'
                                    ) {
                                        self.advance(&chars, 1);
                                        numeric_literal.push(next_char);
                                        break;
                                    }
                                    if next_char == '.' {
                                        if has_dot {
                                            break;
                                        }
                                        has_dot = true;
                                        self.advance(&chars, 1);
                                        numeric_literal.push(next_char);
                                    } else if next_char.is_ascii_digit() {
                                        self.advance(&chars, 1);
                                        numeric_literal.push(next_char);
                                    } else if next_char == '_' {
                                        self.advance(&chars, 1);
                                    } else {
                                        break;
                                    }
                                }

                                Token::Literal(Literal::Numeric(numeric_literal))
                            }
                        }
                    }
                    _ => Token::Operator(Operator::Minus),
                },
                '+' => match chars.get(self.position) {
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::AddAssign)
                    }
                    _ => Token::Operator(Operator::Plus),
                },
                '*' => match chars.get(self.position) {
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::MulAssign)
                    }
                    _ => Token::Operator(Operator::Asterisk),
                },
                '/' => match chars.get(self.position) {
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::DivAssign)
                    }

                    Some(&'/') => {
                        while let Some(&next_char) = chars.get(self.position) {
                            self.advance(&chars, 1);
                            if next_char == '\n' {
                                break;
                            }
                        }
                        continue;
                    }

                    Some(&'*') => {
                        self.advance(&chars, 1);
                        let mut depth: usize = 1;
                        while depth > 0 {
                            let Some(&next_char) = chars.get(self.position) else {
                                anyhow::bail!("Unterminated block comment starting at {}", span);
                            };
                            self.advance(&chars, 1);
                            if next_char == '/' && chars.get(self.position) == Some(&'*') {
                                self.advance(&chars, 1);
                                depth += 1;
                            } else if next_char == '*' && chars.get(self.position) == Some(&'/') {
                                self.advance(&chars, 1);
                                depth -= 1;
                            }
                        }
                        continue;
                    }
                    _ => Token::Operator(Operator::Slash),
                },
                '%' => match chars.get(self.position) {
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::ModAssign)
                    }
                    _ => Token::Operator(Operator::Percent),
                },
                '!' => match chars.get(self.position) {
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::NotEqual)
                    }
                    _ => Token::Operator(Operator::Exclem),
                },
                '>' => match chars.get(self.position) {
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::GreaterEqual)
                    }
                    Some(&'>') => {
                        self.advance(&chars, 1);
                        if chars.get(self.position) == Some(&'=') {
                            self.advance(&chars, 1);
                            Token::Operator(Operator::RShiftAssign)
                        } else {
                            Token::Operator(Operator::RShift)
                        }
                    }
                    _ => Token::Operator(Operator::Greater),
                },
                '<' => match chars.get(self.position) {
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::LessEqual)
                    }
                    Some(&'<') => {
                        self.advance(&chars, 1);
                        if chars.get(self.position) == Some(&'=') {
                            self.advance(&chars, 1);
                            Token::Operator(Operator::LShiftAssign)
                        } else {
                            Token::Operator(Operator::LShift)
                        }
                    }
                    _ => Token::Operator(Operator::Less),
                },
                '&' => match chars.get(self.position) {
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::BitAndAssign)
                    }
                    Some(&'&') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::LogicalAnd)
                    }
                    _ => Token::Operator(Operator::Ampersand),
                },
                '|' => match chars.get(self.position) {
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::BitOrAssign)
                    }
                    Some(&'|') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::LogicalOr)
                    }
                    _ => Token::Operator(Operator::Pipe),
                },
                '^' => match chars.get(self.position) {
                    Some(&'=') => {
                        self.advance(&chars, 1);
                        Token::Operator(Operator::BitXorAssign)
                    }
                    _ => Token::Operator(Operator::Xor),
                },
                '?' => Token::Operator(Operator::Question),
                '\'' => {
                    let mut char_literal = String::new();
                    let mut found_closing = false;
                    while let Some(&next_char) = chars.get(self.position) {
                        self.advance(&chars, 1);
                        if next_char == '\'' {
                            found_closing = true;
                            break;
                        }
                        if next_char == '\\' {
                            if let Some(&escaped_char) = chars.get(self.position) {
                                self.advance(&chars, 1);
                                char_literal.push(match escaped_char {
                                    'n' => '\n',
                                    'r' => '\r',
                                    't' => '\t',
                                    '\\' => '\\',
                                    '\'' => '\'',
                                    '"' => '"',
                                    '0' => '\0',
                                    other => anyhow::bail!(
                                        "Invalid escape sequence: \\{} at {}",
                                        other,
                                        span
                                    ),
                                });
                            } else {
                                anyhow::bail!("Unterminated character literal at {}", span);
                            }
                        } else {
                            char_literal.push(next_char);
                        }
                    }
                    if !found_closing {
                        anyhow::bail!("Unterminated character literal at {}", span);
                    }
                    if char_literal.len() != 1 {
                        anyhow::bail!("Invalid character literal: '{}' at {}", char_literal, span);
                    }
                    Token::Literal(Literal::Char(char_literal.chars().next().unwrap()))
                }
                '\"' => {
                    let mut string_literal = String::new();
                    let mut found_closing = false;
                    while let Some(&next_char) = chars.get(self.position) {
                        self.advance(&chars, 1);
                        if next_char == '\"' {
                            found_closing = true;
                            break;
                        }
                        if next_char == '\\' {
                            if let Some(&escaped_char) = chars.get(self.position) {
                                self.advance(&chars, 1);
                                string_literal.push(match escaped_char {
                                    'n' => '\n',
                                    'r' => '\r',
                                    't' => '\t',
                                    '\\' => '\\',
                                    '\'' => '\'',
                                    '"' => '"',
                                    '0' => '\0',
                                    other => anyhow::bail!(
                                        "Invalid escape sequence: \\{} at {}",
                                        other,
                                        span
                                    ),
                                });
                            } else {
                                anyhow::bail!("Unterminated string literal at {}", span);
                            }
                        } else {
                            string_literal.push(next_char);
                        }
                    }
                    if !found_closing {
                        anyhow::bail!("Unterminated string literal at {}", span);
                    }
                    Token::Literal(Literal::String(string_literal))
                }
                '0'..='9' => {
                    let mut numeric_literal = String::new();
                    numeric_literal.push(c);

                    if c == '0'
                        && let Some(&prefix) = chars.get(self.position)
                        && matches!(prefix, 'x' | 'b' | 'o')
                    {
                        self.advance(&chars, 1);
                        numeric_literal.push(prefix);
                        let validator: fn(char) -> bool = match prefix {
                            'x' => |ch| ch.is_ascii_hexdigit(),
                            'b' => |ch| ch == '0' || ch == '1',
                            'o' => |ch| ('0'..='7').contains(&ch),
                            _ => unreachable!(),
                        };
                        while let Some(&digit) = chars.get(self.position) {
                            if validator(digit) {
                                self.advance(&chars, 1);
                                numeric_literal.push(digit);
                            } else if digit == '_' {
                                self.advance(&chars, 1);
                            } else if digit.is_ascii_whitespace()
                                || matches!(
                                    digit,
                                    '(' | ')'
                                        | '{'
                                        | '}'
                                        | '['
                                        | ']'
                                        | ';'
                                        | ':'
                                        | ','
                                        | '.'
                                        | '+'
                                        | '-'
                                        | '*'
                                        | '/'
                                        | '%'
                                        | '='
                                        | '!'
                                        | '<'
                                        | '>'
                                        | '&'
                                        | '|'
                                        | '^'
                                        | '?'
                                        | '\''
                                        | '"'
                                )
                            {
                                break;
                            } else {
                                anyhow::bail!(
                                    "Invalid {} literal: {}{} at {}",
                                    match prefix {
                                        'x' => "hexadecimal",
                                        'b' => "binary",
                                        'o' => "octal",
                                        _ => unreachable!(),
                                    },
                                    numeric_literal,
                                    digit,
                                    span
                                );
                            }
                        }

                        Token::Literal(Literal::Numeric(numeric_literal))
                    } else {
                        let mut has_dot = false;
                        while let Some(&next_char) = chars.get(self.position) {
                            if matches!(next_char, 'd' | 'D' | 'f' | 'F' | 'u' | 'U' | 'l' | 'L') {
                                self.advance(&chars, 1);
                                numeric_literal.push(next_char);
                                break;
                            }
                            if next_char == '.' {
                                if has_dot {
                                    break;
                                }
                                has_dot = true;
                                self.advance(&chars, 1);
                                numeric_literal.push(next_char);
                            } else if next_char.is_ascii_digit() {
                                self.advance(&chars, 1);
                                numeric_literal.push(next_char);
                            } else if next_char == '_' {
                                self.advance(&chars, 1);
                            } else {
                                break;
                            }
                        }

                        Token::Literal(Literal::Numeric(numeric_literal))
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut identifier = String::new();
                    identifier.push(c);
                    while let Some(&next_char) = chars.get(self.position) {
                        if next_char.is_ascii_alphanumeric() || next_char == '_' {
                            self.advance(&chars, 1);
                            identifier.push(next_char);
                        } else {
                            break;
                        }
                    }
                    match identifier.as_str() {
                        "true" => Token::Literal(Literal::Boolean(true)),
                        "false" => Token::Literal(Literal::Boolean(false)),
                        "as" | "break" | "const" | "continue" | "else" | "enum" | "extern"
                        | "fn" | "for" | "if" | "in" | "let" | "loop" | "match" | "mod"
                        | "static" | "struct" | "type" | "use" | "while" => {
                            Token::Keyword(Keyword::from(&identifier)?)
                        }
                        _ => Token::Identifier(identifier),
                    }
                }

                _ => anyhow::bail!("Unexpected character: '{}' at {}", c, span),
            };
            self.tokens.push((token, span));
        }

        let eof_span = Span {
            line: self.line,
            col: self.col,
        };
        self.tokens.push((Token::Eof, eof_span));

        Ok(())
    }
}
