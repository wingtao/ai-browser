use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Let,
    Const,
    Var,
    Function,
    Return,
    If,
    Else,
    While,
    For,
    Break,
    Continue,
    True,
    False,
    Null,
    Undefined,
    Identifier(String),
    Number(f64),
    String(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Semicolon,
    Colon,
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    EqEq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    AndAnd,
    OrOr,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("unexpected character `{ch}` at {pos}")]
    UnexpectedChar { ch: char, pos: usize },
    #[error("unterminated string at {pos}")]
    UnterminatedString { pos: usize },
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let chars = input.chars().collect::<Vec<_>>();
    let mut i = 0usize;
    let mut tokens = Vec::new();

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        let make = |kind| Token { kind, pos: i };
        match c {
            '(' => {
                tokens.push(make(TokenKind::LParen));
                i += 1;
            }
            ')' => {
                tokens.push(make(TokenKind::RParen));
                i += 1;
            }
            '{' => {
                tokens.push(make(TokenKind::LBrace));
                i += 1;
            }
            '}' => {
                tokens.push(make(TokenKind::RBrace));
                i += 1;
            }
            ',' => {
                tokens.push(make(TokenKind::Comma));
                i += 1;
            }
            '.' => {
                tokens.push(make(TokenKind::Dot));
                i += 1;
            }
            ';' => {
                tokens.push(make(TokenKind::Semicolon));
                i += 1;
            }
            ':' => {
                tokens.push(make(TokenKind::Colon));
                i += 1;
            }
            '+' => {
                tokens.push(make(TokenKind::Plus));
                i += 1;
            }
            '-' => {
                tokens.push(make(TokenKind::Minus));
                i += 1;
            }
            '*' => {
                tokens.push(make(TokenKind::Star));
                i += 1;
            }
            '/' => {
                if chars.get(i + 1) == Some(&'/') {
                    while i < chars.len() && chars[i] != '\n' {
                        i += 1;
                    }
                } else {
                    tokens.push(make(TokenKind::Slash));
                    i += 1;
                }
            }
            '=' => {
                if chars.get(i + 1) == Some(&'=') {
                    tokens.push(make(TokenKind::EqEq));
                    i += 2;
                } else {
                    tokens.push(make(TokenKind::Assign));
                    i += 1;
                }
            }
            '!' => {
                if chars.get(i + 1) == Some(&'=') {
                    tokens.push(make(TokenKind::NotEq));
                    i += 2;
                } else {
                    tokens.push(make(TokenKind::Bang));
                    i += 1;
                }
            }
            '>' => {
                if chars.get(i + 1) == Some(&'=') {
                    tokens.push(make(TokenKind::Gte));
                    i += 2;
                } else {
                    tokens.push(make(TokenKind::Gt));
                    i += 1;
                }
            }
            '<' => {
                if chars.get(i + 1) == Some(&'=') {
                    tokens.push(make(TokenKind::Lte));
                    i += 2;
                } else {
                    tokens.push(make(TokenKind::Lt));
                    i += 1;
                }
            }
            '&' => {
                if chars.get(i + 1) == Some(&'&') {
                    tokens.push(make(TokenKind::AndAnd));
                    i += 2;
                } else {
                    return Err(LexError::UnexpectedChar { ch: c, pos: i });
                }
            }
            '|' => {
                if chars.get(i + 1) == Some(&'|') {
                    tokens.push(make(TokenKind::OrOr));
                    i += 2;
                } else {
                    return Err(LexError::UnexpectedChar { ch: c, pos: i });
                }
            }
            '"' | '\'' => {
                let quote = c;
                let start = i;
                i += 1;
                let mut out = String::new();
                while i < chars.len() && chars[i] != quote {
                    if chars[i] == '\\' {
                        i += 1;
                        if i >= chars.len() {
                            break;
                        }
                        out.push(chars[i]);
                    } else {
                        out.push(chars[i]);
                    }
                    i += 1;
                }
                if i >= chars.len() {
                    return Err(LexError::UnterminatedString { pos: start });
                }
                i += 1;
                tokens.push(Token {
                    kind: TokenKind::String(out),
                    pos: start,
                });
            }
            _ if c.is_ascii_digit() => {
                let start = i;
                i += 1;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let num = chars[start..i].iter().collect::<String>();
                let number = num.parse::<f64>().unwrap_or(0.0);
                tokens.push(Token {
                    kind: TokenKind::Number(number),
                    pos: start,
                });
            }
            _ if c.is_ascii_alphabetic() || c == '_' => {
                let start = i;
                i += 1;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let ident = chars[start..i].iter().collect::<String>();
                let kind = match ident.as_str() {
                    "let" => TokenKind::Let,
                    "const" => TokenKind::Const,
                    "var" => TokenKind::Var,
                    "function" => TokenKind::Function,
                    "return" => TokenKind::Return,
                    "if" => TokenKind::If,
                    "else" => TokenKind::Else,
                    "while" => TokenKind::While,
                    "for" => TokenKind::For,
                    "break" => TokenKind::Break,
                    "continue" => TokenKind::Continue,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "null" => TokenKind::Null,
                    "undefined" => TokenKind::Undefined,
                    _ => TokenKind::Identifier(ident),
                };
                tokens.push(Token { kind, pos: start });
            }
            _ => return Err(LexError::UnexpectedChar { ch: c, pos: i }),
        }
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        pos: input.len(),
    });
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_function_and_call() {
        let code = "function add(a,b){ return a+b; } add(1,2);";
        let tokens = lex(code).unwrap();
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Function)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Return)));
    }

    #[test]
    fn lex_for_break_continue() {
        let code = "for(let i=0;i<3;i=i+1){ if(i==1){continue;} break; }";
        let tokens = lex(code).unwrap();
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::For)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Break)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Continue)));
    }
}
