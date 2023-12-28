use logos::{Lexer, Logos};

/// A list of C tokens that are used in the parser.
#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a> {
    #[token("int")]
    Int,
    #[token("char")]
    Char,
    #[token("long")]
    Long,
    #[token("short")]
    Short,
    #[token("float")]
    Float,
    #[token("double")]
    Double,
    #[token("void")]
    Void,
    #[token("struct")]
    Struct,
    #[token("union")]
    Union,
    #[token("enum")]
    Enum,
    #[token("signed")]
    Signed,
    #[token("unsigned")]
    Unsigned,
    #[token("const")]
    Const,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("do")]
    Do,
    #[token("while")]
    While,
    #[token("if")]
    If,
    #[token("return")]
    Return,
    #[token("slog")]
    Slog(&'a str),
}

// Handlers
/// Handler to get inner string of log function
fn slog<'a>(lex: &'a mut Lexer<Token>) -> Option<&'a str> {
    let slice = lex.slice();
    let mut left: usize = 0;
    let mut right: usize = slice.len() - 1;
    let mut db: bool = false;
    let (mut db1, mut db2) = (false, false);
    for (i, c) in slice.chars().enumerate() {
        if c == r#"""# {
            if !db {
                left = i;
            } else {
                db = true;
                right = i;
            }
            if db1 == false {
                db1 == true;
            } else {
                db2 == true
            }
        }
    }
    db2.then(|| db1.then(|| &slice[left..=right]).unwrap())
}
